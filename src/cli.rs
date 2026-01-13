use crate::{editor, note, search, traversal};
use chrono::Utc;
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell as CompletionShell};

use home_dir::HomeDirExt;
use std::io;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A simple note-taking CLI for managing markdown notes",
    long_about = "noteorg - Organize and search your markdown notes with ease.\n\nFeatures:\n  - List all notes with metadata\n  - Search through note content, titles, tags, and categories\n  - Edit notes directly in your editor"
)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// List all notes with their metadata (title, tags, category, date)
    List {
        /// Path to notes directory (default: ~/Notes/)
        #[arg(help = "Custom path to search for notes")]
        path: Option<String>,
    },

    /// Edit notes matching a regex pattern
    Edit {
        /// Regex pattern to search for in note content, title, tags, and filename
        #[arg(help = "Regular expression to match notes")]
        search_value: Option<String>,
    },

    /// Interactive search through all notes with real-time results
    Search,

    /// Show statistics about your notes (not yet implemented)
    Statistics,

    /// Generate shell completion scripts
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

pub fn list_files(path: Option<String>) -> io::Result<()> {
    let base_path = match path {
        Some(value) => value,
        None => "~/Notes/"
            .expand_home()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    };
    list_files_internal(&base_path)
}

fn list_files_internal(base_path: &str) -> io::Result<()> {
    let files = traversal::get_files(Path::new(base_path));
    let note_vec: Vec<note::Note<Utc>> = files
        .unwrap_or_default()
        .iter()
        .filter(|&x| x.to_string_lossy().contains(".md"))
        .map(|file| {
            note::read_note(file, Path::new("./tests/dir_structure_example/"), &Utc).unwrap()
        })
        .collect();
    for note in note_vec {
        // Format category as path, filtering out "Notes"
        let category_parts: Vec<&String> = note.metadata.category
            .iter()
            .filter(|&part| part != "Notes")
            .collect();

        let category = if category_parts.is_empty() {
            String::from("root")
        } else {
            category_parts.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("/")
        };

        // Format tags
        let tags = if note.metadata.tags.is_empty() {
            String::new()
        } else {
            format!("#{}",note.metadata.tags.join(" #"))
        };

        // Format date (just the date part, not time)
        let modified = note.metadata.date_last_modified.format("%Y-%m-%d");

        // Print in format: [category/path] Title #tags (date)
        if tags.is_empty() {
            println!("[{}] {} ({})", category, note.metadata.title, modified);
        } else {
            println!("[{}] {} {} ({})", category, note.metadata.title, tags, modified);
        }
    }
    Ok(())
}

pub fn edit_file(search_value: Option<String>) -> io::Result<()> {
    match search_value {
        Some(value) => edit_file_internal(&value),
        None => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not a valid path",
        )),
    }
}

fn edit_file_internal(search_value: &str) -> io::Result<()> {
    let base_path = "~/Notes/".expand_home().unwrap();
    let matched_files = search::search_files(search_value, base_path.as_path())?;

    if matched_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No file found matching regex: {}", search_value),
        ));
    }

    editor::launch_editor(&matched_files)?;
    Ok(())
}

pub fn show_search_results_realtime() -> io::Result<()> {
    let base_path = "~/Notes/".expand_home().unwrap();
    search::show_search_results_realtime(base_path.as_path())
}

pub fn generate_completions(shell: Shell) {
    let mut cmd = Args::command();

    let completion_shell = match shell {
        Shell::Bash => CompletionShell::Bash,
        Shell::Zsh => CompletionShell::Zsh,
        Shell::Fish => CompletionShell::Fish,
        Shell::PowerShell => CompletionShell::PowerShell,
        Shell::Elvish => CompletionShell::Elvish,
    };

    generate(completion_shell, &mut cmd, "note", &mut io::stdout());
}
