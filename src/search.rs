use regex::Regex;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::{note, traversal};
use chrono::Utc;

use crossterm::{
    cursor, execute,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};

struct SearchState {
    input: String,
    last_input: String,
    selected_index: usize,
    last_selected: usize,
    current_results: Vec<PathBuf>,
    needs_redraw: bool,
}

impl SearchState {
    fn new() -> Self {
        Self {
            input: String::new(),
            last_input: String::from("__FORCE_DRAW__"),
            selected_index: 0,
            last_selected: usize::MAX,
            current_results: Vec::new(),
            needs_redraw: true,
        }
    }

    fn has_changes(&self) -> bool {
        self.input != self.last_input
            || self.selected_index != self.last_selected
            || self.needs_redraw
    }

    fn mark_rendered(&mut self) {
        self.last_input = self.input.clone();
        self.last_selected = self.selected_index;
        self.needs_redraw = false;
    }
}

pub fn search_files(
    search_value: &str,
    base_path: &Path,
) -> io::Result<Vec<PathBuf>> {
    let files = traversal::get_files(base_path)?;

    let re = Regex::new(search_value).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid regex: {}", e))
    })?;

    let mut matching_files = Vec::new();

    for path in files {
        // Only process .md files
        let is_markdown = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "md")
            .unwrap_or(false);

        if !is_markdown {
            continue;
        }

        // Read the note to search in metadata and content
        match note::read_note(&path, base_path, &Utc) {
            Ok(note) => {
                // Concatenate all searchable fields
                let searchable_text = format!(
                    "{} {} {} {} {}",
                    note.metadata.filename,
                    note.metadata.title,
                    note.metadata.tags.join(" "),
                    note.metadata.category.join(" "),
                    note.content
                );

                // Check if the regex matches anywhere in the concatenated text
                if re.is_match(&searchable_text) {
                    matching_files.push(path);
                }
            }
            Err(_) => {
                // If we can't read the note, skip it silently
                continue;
            }
        }
    }

    Ok(matching_files)
}

fn render_search_input<W: Write>(stdout: &mut W, input: &str) -> io::Result<()> {
    execute!(stdout, cursor::MoveTo(0, 0))?;
    write!(stdout, "Search: {}", input)?;
    Ok(())
}

fn render_separator<W: Write>(stdout: &mut W) -> io::Result<()> {
    execute!(stdout, cursor::MoveTo(0, 1))?;
    write!(stdout, "{}", "─".repeat(50))?;
    Ok(())
}

fn render_results<W: Write>(
    stdout: &mut W,
    state: &mut SearchState,
    base_path: &Path,
) -> io::Result<()> {
    if state.input.is_empty() {
        execute!(stdout, cursor::MoveTo(0, 2))?;
        write!(stdout, "Start typing to search...")?;
        state.current_results.clear();
        state.selected_index = 0;
        return Ok(());
    }

    match search_files(&state.input, base_path) {
        Ok(files) => {
            let input_changed = state.input != state.last_input;
            if input_changed {
                state.current_results = files;
                state.selected_index = 0;
            }

            if state.current_results.is_empty() {
                execute!(stdout, cursor::MoveTo(0, 2))?;
                write!(stdout, "No matches")?;
            } else {
                execute!(stdout, cursor::MoveTo(0, 2))?;
                write!(
                    stdout,
                    "Found {} matches (↑↓ to select, Enter to edit):",
                    state.current_results.len()
                )?;

                for (i, file) in state.current_results.iter().enumerate().take(10) {
                    let filename = file
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");

                    execute!(stdout, cursor::MoveTo(0, 3 + i as u16))?;

                    if i == state.selected_index {
                        write!(
                            stdout,
                            "{}▶ {}. {}{}",
                            SetForegroundColor(Color::Cyan),
                            i + 1,
                            filename,
                            ResetColor
                        )?;
                    } else {
                        write!(stdout, "  {}. {}", i + 1, filename)?;
                    }
                }
            }
        }
        Err(_) => {
            execute!(stdout, cursor::MoveTo(0, 2))?;
            write!(stdout, "Invalid regex pattern")?;
            state.current_results.clear();
        }
    }

    Ok(())
}

fn render_search_ui<W: Write>(
    stdout: &mut W,
    state: &mut SearchState,
    base_path: &Path,
) -> io::Result<()> {
    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))?;
    render_search_input(stdout, &state.input)?;
    render_separator(stdout)?;
    render_results(stdout, state, base_path)?;
    stdout.flush()?;
    Ok(())
}

enum SearchAction {
    Continue,
    Exit,
    OpenEditor(PathBuf),
    MoveUp,
    MoveDown,
    AddChar(char),
    DeleteChar,
    ClearAndExit,
}

fn handle_keyboard_event(key: KeyEvent, state: &SearchState) -> SearchAction {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            SearchAction::Exit
        }
        KeyCode::Up => SearchAction::MoveUp,
        KeyCode::Down => SearchAction::MoveDown,
        KeyCode::Enter => {
            if !state.current_results.is_empty() && state.selected_index < state.current_results.len() {
                SearchAction::OpenEditor(state.current_results[state.selected_index].clone())
            } else {
                SearchAction::Continue
            }
        }
        KeyCode::Char(c) => SearchAction::AddChar(c),
        KeyCode::Backspace => SearchAction::DeleteChar,
        KeyCode::Esc => SearchAction::ClearAndExit,
        _ => SearchAction::Continue,
    }
}

fn apply_action(action: SearchAction, state: &mut SearchState) -> Option<PathBuf> {
    match action {
        SearchAction::MoveUp => {
            if state.selected_index > 0 {
                state.selected_index -= 1;
            }
            None
        }
        SearchAction::MoveDown => {
            if state.selected_index < state.current_results.len().saturating_sub(1) {
                state.selected_index += 1;
            }
            None
        }
        SearchAction::AddChar(c) => {
            state.input.push(c);
            None
        }
        SearchAction::DeleteChar => {
            state.input.pop();
            None
        }
        SearchAction::OpenEditor(path) => Some(path),
        _ => None,
    }
}

pub fn show_search_results_realtime(base_path: &Path) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let mut state = SearchState::new();
    let mut should_exit = false;

    while !should_exit {
        if state.has_changes() {
            render_search_ui(&mut stdout, &mut state, base_path)?;
            state.mark_rendered();
        }

        if event::poll(std::time::Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
        {
            let action = handle_keyboard_event(key, &state);

            match action {
                SearchAction::Exit | SearchAction::ClearAndExit => {
                    should_exit = true;
                }
                SearchAction::OpenEditor(path) => {
                    disable_raw_mode()?;
                    execute!(stdout, LeaveAlternateScreen)?;

                    crate::editor::launch_editor(&[path])?;

                    enable_raw_mode()?;
                    execute!(stdout, EnterAlternateScreen)?;
                    state.needs_redraw = true;
                }
                _ => {
                    apply_action(action, &mut state);
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
