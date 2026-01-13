# noteorg

A simple command-line tool for managing and searching markdown notes.

## Author

**Ammar Mian**

This project was created as a learning exercise to practice Rust programming. Claude Code assisted specifically with crossterm terminal UI implementation and regex pattern matching.

## Features

- **List notes** - Display all markdown notes with metadata (title, tags, category, last modified date)
- **Search notes** - Interactive real-time search through note content, titles, tags, and categories using regex
- **Edit notes** - Quickly open notes in your editor by pattern matching
- **Shell completions** - Tab completion support for Bash, Zsh, Fish, PowerShell, and Elvish

## Installation

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))
- A text editor (defaults to `nvim`)

### Install

```bash
# Clone the repository
cd noteorg

# Install the binary
cargo install --path .

# Generate shell completions (for zsh)
mkdir -p ~/.zsh/completions
note completions zsh > ~/.zsh/completions/_note

# Add to ~/.zshrc (if not already present):
# fpath=(~/.zsh/completions $fpath)
# autoload -Uz compinit && compinit
```

## Usage

### List all notes

```bash
note list
# Output: [category/path] Note Title #tag1 #tag2 (2024-01-13)

# List notes in a specific directory
note list ~/Documents/MyNotes
```

### Interactive search

```bash
note search
```

Opens an interactive search interface where you can:
- Type a regex pattern to search through all note content and metadata
- Use arrow keys to navigate results
- Press Enter to open the selected note in your editor
- Press Esc or Ctrl+C to exit

### Edit notes by pattern

```bash
# Edit notes matching a regex pattern
note edit "meeting"

# Edit notes with specific tags
note edit "urgent"

# Multiple matches will all open in your editor
```

### Generate shell completions

```bash
# For Zsh
note completions zsh > ~/.zsh/completions/_note

# For Bash
note completions bash > /usr/local/etc/bash_completion.d/note

# For Fish
note completions fish > ~/.config/fish/completions/note.fish
```

## Configuration

By default, noteorg looks for notes in `~/Notes/`. The notes are expected to be:
- Markdown files (`.md` extension)
- Optionally with YAML frontmatter for metadata:

```yaml
---
title: My Note Title
tags: [rust, cli, learning]
date: 2024-01-13
---

Note content here...
```

## Project Structure

```
noteorg/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Module declarations
│   ├── cli.rs           # CLI argument parsing and command handlers
│   ├── editor.rs        # Editor launching utilities
│   ├── search.rs        # Search functionality with interactive UI
│   ├── note.rs          # Note parsing and metadata extraction
│   └── traversal.rs     # File system traversal
├── Cargo.toml           # Dependencies and project configuration
└── README.md            # This file
```

## Development

This is a learning project for practicing Rust. Key learning areas:

- CLI argument parsing with `clap`
- File system operations
- Terminal UI with `crossterm` (assisted by Claude Code)
- Regex pattern matching (assisted by Claude Code)
- YAML frontmatter parsing
- Code organization and modularization
- Error handling with `Result<T>`

### Build from source

```bash
# Development build
cargo build

# Run tests
cargo test

# Run with cargo
cargo run -- list

# Release build
cargo build --release
```

### Code Quality

```bash
# Run clippy for linting
cargo clippy

# Format code
cargo fmt
```

## License

This is a personal learning project. Feel free to use and modify as needed.

