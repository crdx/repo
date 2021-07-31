use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_repos(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    recurse(&mut paths, path)?;
    Ok(paths)
}

fn recurse(paths: &mut Vec<PathBuf>, path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if entry.file_name() == ".git" {
                paths.push(path.to_path_buf());
            } else if let Err(e) = recurse(paths, &entry.path()) {
                // Fine to silently ignore inaccessible directories.
                if e.kind() != io::ErrorKind::PermissionDenied {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}
