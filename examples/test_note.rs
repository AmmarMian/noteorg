use chrono::Utc;
use noteorg::note::{Note, NoteMetadata, read_note, read_note_metadata};
use noteorg::traversal::get_files;
use std::{io, path::Path};

fn main() -> io::Result<()> {
    // let path = Path::new("./tests/dir_structure_example/A/C/C.out");
    // let metadata = read_note_metadata(path, Path::new("./tests/dir_structure_example"), &Utc);
    let files = get_files(Path::new("./tests/dir_structure_example/"));
    // let metadata_vec: Vec<NoteMetadata<Utc>> = files
    //     .unwrap_or_default()
    //     .iter()
    //     .map(|file| {
    //         read_note_metadata(
    //             file.as_path(),
    //             Path::new("./tests/dir_structure_example/"),
    //             &Utc,
    //         )
    //         .unwrap()
    //     })
    //     .collect();
    //
    // for metadata in metadata_vec {
    //     println!("Parsed metadata : {:?}\n", metadata);
    // }
    //
    let note_vec: Vec<Note<Utc>> = files
        .unwrap_or_default()
        .iter()
        .map(|file| read_note(file, Path::new("./tests/dir_structure_example/"), &Utc).unwrap())
        .collect();
    for note in note_vec {
        println!("Parsed note {:?}\n\n", note);
    }

    Ok(())
}
