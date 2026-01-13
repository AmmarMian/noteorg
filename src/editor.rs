use std::io;
use std::path::PathBuf;
use std::process::Command;

/// Launch neovim with the provided files
pub fn launch_editor(files: &[PathBuf]) -> io::Result<()> {
    if files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No files provided to editor",
        ));
    }

    let mut cmd = Command::new("nvim");
    for file in files {
        cmd.arg(file);
    }
    cmd.status()?;
    Ok(())
}
