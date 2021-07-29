use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_repos(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    recurse(&mut paths, path.to_path_buf())?;
    Ok(paths)
}

fn recurse(paths: &mut Vec<PathBuf>, path: PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            // If this is the .git directory then we're done. No need to
            // recurse into any more directories at this level since
            // this is (likely) a repository and repositories shouldn't
            // be nested.
            if entry.file_name() == ".git" {
                paths.push(path);
                break;
            }

            // Go deeper.
            if let Err(e) = recurse(paths, entry.path()) {
                // Fine to silently ignore inaccessible directories.
                if e.kind() != io::ErrorKind::PermissionDenied {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}
