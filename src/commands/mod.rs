
pub mod add;
pub mod apply;
pub mod diff;
pub mod init;
pub mod push;
pub mod remove;
pub mod sync;

use crate::output::warning;
use std::fs;
use std::path::Path;

/// Copies `src` to `dst`, handling both files and directories.
pub(super) fn copy_entry(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        copy_dir_all(src, dst)
    } else {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, dst)?;
        Ok(())
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_name() == ".git" {
            continue;
        }
        // Use symlink_metadata so we see the link itself, not its target.
        let metadata = entry.path().symlink_metadata()?;
        let file_type = metadata.file_type();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_symlink() {
            // Resolve the symlink and copy its actual content.
            let resolved = fs::canonicalize(entry.path());
            match resolved {
                Ok(real) if real.is_dir() => {
                    if let Err(e) = copy_dir_all(&real, &dst_path) {
                        warning!("skipping symlink '{}': {}", entry.path().display(), e);
                    }
                }
                Ok(real) => {
                    if let Err(e) = fs::copy(&real, &dst_path) {
                        warning!("skipping symlink '{}': {}", entry.path().display(), e);
                    }
                }
                Err(e) => {
                    warning!("skipping unresolvable symlink '{}': {}", entry.path().display(), e);
                }
            }
        } else if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            if let Err(e) = fs::copy(entry.path(), &dst_path) {
                warning!("skipping '{}': {}", entry.path().display(), e);
            }
        }
    }
    Ok(())
}

