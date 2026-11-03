pub mod add;
pub mod apply;
pub mod diff;
pub mod ignore;
pub mod init;
pub mod pull;
pub mod push;
pub mod remove;
pub mod sync;

use crate::output::warning;
use ::ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::fs;
use std::path::{Path, PathBuf};

pub const DOTIGNORE_FILENAME: &str = ".dotignore";

/// Loads `<target>/.dotignore` (gitignore syntax, patterns relative to the repo root).
/// Returns `None` if the file does not exist or cannot be built.
pub(super) fn load_ignore(target: &Path) -> Option<Gitignore> {
    let file = target.join(DOTIGNORE_FILENAME);
    if !file.is_file() {
        return None;
    }
    let mut builder = GitignoreBuilder::new(target);
    if let Some(e) = builder.add(&file) {
        warning!("some patterns in {} are invalid: {}", file.display(), e);
    }
    builder.build().ok()
}

/// True if the repo-relative path (e.g. `nvim/lazy-lock.json`) is ignored.
pub(super) fn is_ignored(ignore: Option<&Gitignore>, rel: &Path, is_dir: bool) -> bool {
    ignore.is_some_and(|g| g.matched(rel, is_dir).is_ignore())
}

/// True if `link` is a symlink that resolves to the same path as `source`.
pub(super) fn is_link_to(link: &Path, source: &Path) -> bool {
    let is_symlink = link
        .symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);
    if !is_symlink {
        return false;
    }
    match (fs::canonicalize(link), fs::canonicalize(source)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

/// Removes a file, directory or symlink (without following links).
pub(super) fn remove_path(path: &Path) -> std::io::Result<()> {
    let meta = path.symlink_metadata()?;
    if meta.file_type().is_symlink() {
        // Directory symlinks on Windows need remove_dir.
        fs::remove_file(path).or_else(|_| fs::remove_dir(path))
    } else if meta.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

/// Copies `src` to `dst`, handling both files and directories.
pub(super) fn copy_entry(src: &Path, dst: &Path) -> std::io::Result<()> {
    copy_entry_filtered(src, dst, None, Path::new(""))
}

/// Like [`copy_entry`], but skips paths matched by `ignore`. `rel` is the
/// repo-relative path of `src`/`dst` (the entry name, e.g. `nvim`).
pub(super) fn copy_entry_filtered(
    src: &Path,
    dst: &Path,
    ignore: Option<&Gitignore>,
    rel: &Path,
) -> std::io::Result<()> {
    if src.is_dir() {
        copy_dir_all(src, dst, ignore, rel)
    } else {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, dst)?;
        Ok(())
    }
}

fn copy_dir_all(src: &Path, dst: &Path, ignore: Option<&Gitignore>, rel: &Path) -> std::io::Result<()> {
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
        let rel_path: PathBuf = rel.join(entry.file_name());

        let is_dir = if file_type.is_symlink() {
            fs::canonicalize(entry.path()).map(|p| p.is_dir()).unwrap_or(false)
        } else {
            file_type.is_dir()
        };
        if is_ignored(ignore, &rel_path, is_dir) {
            continue;
        }

        if file_type.is_symlink() {
            // Resolve the symlink and copy its actual content.
            let resolved = fs::canonicalize(entry.path());
            match resolved {
                Ok(real) if real.is_dir() => {
                    if let Err(e) = copy_dir_all(&real, &dst_path, ignore, &rel_path) {
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
            copy_dir_all(&entry.path(), &dst_path, ignore, &rel_path)?;
        } else {
            if let Err(e) = fs::copy(entry.path(), &dst_path) {
                warning!("skipping '{}': {}", entry.path().display(), e);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("dot-rs-test-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn ignore_matches_repo_relative_paths() {
        let repo = tmp("match");
        fs::write(repo.join(DOTIGNORE_FILENAME), "nvim/lazy-lock.json\n*.log\n**/cache/\n").unwrap();
        let g = load_ignore(&repo);
        let g = g.as_ref();
        assert!(is_ignored(g, Path::new("nvim/lazy-lock.json"), false));
        assert!(!is_ignored(g, Path::new("other/lazy-lock.json"), false));
        assert!(is_ignored(g, Path::new("a/b/x.log"), false));
        assert!(is_ignored(g, Path::new("nvim/deep/cache"), true));
        assert!(!is_ignored(g, Path::new("nvim/init.lua"), false));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn copy_skips_ignored() {
        let repo = tmp("copy-repo");
        let src = tmp("copy-src");
        fs::write(repo.join(DOTIGNORE_FILENAME), "nvim/lazy-lock.json\n**/cache/\n").unwrap();
        fs::create_dir_all(src.join("cache")).unwrap();
        fs::write(src.join("cache/x"), "x").unwrap();
        fs::write(src.join("lazy-lock.json"), "l").unwrap();
        fs::write(src.join("init.lua"), "i").unwrap();
        let dst = repo.join("nvim");
        let g = load_ignore(&repo);
        copy_entry_filtered(&src, &dst, g.as_ref(), Path::new("nvim")).unwrap();
        assert!(dst.join("init.lua").exists());
        assert!(!dst.join("lazy-lock.json").exists());
        assert!(!dst.join("cache").exists());
        let _ = fs::remove_dir_all(&repo);
        let _ = fs::remove_dir_all(&src);
    }
}
