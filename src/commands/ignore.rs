use std::fs;
use std::path::Path;

use crate::config::resolve_target;
use crate::output::{error, info, success};

use super::DOTIGNORE_FILENAME;

/// Appends `patterns` to `file` (created if missing), skipping exact duplicates.
/// Returns `(added, skipped)`.
pub(super) fn append_patterns(file: &Path, patterns: &[String]) -> std::io::Result<(Vec<String>, Vec<String>)> {
    let mut content = if file.exists() { fs::read_to_string(file)? } else { String::new() };
    let mut added = Vec::new();
    let mut skipped = Vec::new();
    for p in patterns {
        let p = p.trim();
        if p.is_empty() {
            continue;
        }
        if content.lines().any(|l| l.trim() == p) {
            skipped.push(p.to_string());
            continue;
        }
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(p);
        content.push('\n');
        added.push(p.to_string());
    }
    if !added.is_empty() {
        fs::write(file, content)?;
    }
    Ok((added, skipped))
}

/// Adds patterns to `<target>/.dotignore`, or lists them with `--list`.
pub fn ignore(patterns: &[String], list: bool) {
    let file = resolve_target().join(DOTIGNORE_FILENAME);

    if list {
        match fs::read_to_string(&file) {
            Ok(c) => print!("{}", c),
            Err(_) => info!("no {} at {}", DOTIGNORE_FILENAME, file.display()),
        }
        return;
    }

    if patterns.is_empty() {
        error!("no patterns given — usage: dot ignore <PATTERN>... (or --list)");
        std::process::exit(1);
    }

    match append_patterns(&file, patterns) {
        Ok((added, skipped)) => {
            for p in &skipped {
                info!("'{}' is already in {}", p, file.display());
            }
            for p in &added {
                success!("ignored '{}' ({})", p, file.display());
            }
        }
        Err(e) => {
            error!("failed to update {}: {}", file.display(), e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_and_dedup() {
        let dir = std::env::temp_dir().join(format!("dot-rs-test-ign-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let f = dir.join(".dotignore");
        fs::write(&f, "*.log").unwrap(); // no trailing newline
        let (a, s) = append_patterns(&f, &["nvim/x".into(), "*.log".into(), "nvim/x".into()]).unwrap();
        assert_eq!(a, vec!["nvim/x"]);
        assert_eq!(s, vec!["*.log", "nvim/x"]);
        assert_eq!(fs::read_to_string(&f).unwrap(), "*.log\nnvim/x\n");
        let _ = fs::remove_dir_all(&dir);
    }
}
