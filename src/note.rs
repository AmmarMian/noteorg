// Notes abstractions

use chrono::{DateTime, TimeZone, Utc};
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs::{metadata, read_to_string};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct NoteFrontMatter {
    title: Option<String>,
    tags: Option<Vec<String>>,
    date: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct NoteMetadata<Tz: TimeZone> {
    pub filename: String,
    pub title: String,
    pub tags: Vec<String>,
    pub category: Vec<String>,
    pub date_created: DateTime<Tz>,
    pub date_last_modified: DateTime<Tz>,
}

#[derive(Debug)]
pub struct Note<Tz: TimeZone> {
    pub metadata: NoteMetadata<Tz>,
    pub content: String,
    pub path: PathBuf,
}

pub fn read_note_metadata<Tz: TimeZone>(
    path: &Path,
    root_path: &Path,
    tz: &Tz,
) -> io::Result<NoteMetadata<Tz>> {
    if !path.is_file() {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{:?} is not a file", path),
        ))
    } else {
        let metadata = metadata(path)?;
        let last_modified_utc: DateTime<Utc> = metadata.modified()?.into();
        let last_modified = last_modified_utc.with_timezone(tz);
        let created_utc: DateTime<Utc> = metadata.created()?.into();
        let created = created_utc.with_timezone(tz);
        let title: String = path
            .file_name()
            .unwrap_or(OsStr::new("Empty"))
            .to_string_lossy()
            .into_owned();
        let filename = title.clone();
        Ok(NoteMetadata {
            filename,
            title,
            tags: vec![],
            category: path
                .iter()
                .skip(root_path.iter().count())
                .map(|x| x.to_string_lossy().into_owned())
                .filter(|x| !x.contains('.'))
                .collect(),
            date_created: created,
            date_last_modified: last_modified,
        })
    }
}

pub fn read_note<Tz: TimeZone>(path: &Path, root_path: &Path, tz: &Tz) -> io::Result<Note<Tz>> {
    let mut note = Note {
        metadata: read_note_metadata(path, root_path, tz)?,
        content: read_to_string(path)?,
        path: path.to_path_buf(),
    };
    // Parsing frontmatter
    let matter = Matter::<YAML>::new();
    let frontmatter: Option<NoteFrontMatter> = matter
        .parse::<NoteFrontMatter>(note.content.as_str())
        .ok()
        .and_then(|info| info.data);
    if let Some(data) = frontmatter {
        note.metadata.tags = data.tags.unwrap_or_default();
        note.metadata.title = data.title.unwrap_or_default();
    }

    Ok(note)
}
