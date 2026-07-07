use crate::config::{resolve_target, DotEntries, ENTRIES_FILENAME};
use std::process::Command;

/// Shows differences between each tracked entry's copy in the dotfiles
/// folder and its live source location, using `git diff --no-index`.
pub fn diff() {
    let target = resolve_target();
    let entries_path = target.join(ENTRIES_FILENAME);

    let entries = DotEntries::load(&entries_path).unwrap_or_else(|e| {
        eprintln!("error: failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    let mut items = entries.get_entries();
    items.sort_by(|a, b| a.0.cmp(&b.0));

    if items.is_empty() {
        println!("nothing to diff — no entries in {}", entries_path.display());
        return;
    }

    for (name, source) in &items {
        let tracked = target.join(name);

        if !tracked.exists() {
            eprintln!("warning: skipping '{}' — not yet synced: {}", name, tracked.display());
            continue;
        }
        if !source.exists() {
            eprintln!("warning: skipping '{}' — source does not exist: {}", name, source.display());
            continue;
        }

        let status = Command::new("git")
            .arg("diff")
            .arg("--no-index")
            .arg("--")
            .arg(&tracked)
            .arg(source)
            .status();

        match status {
            Ok(status) if status.success() || status.code() == Some(1) => {}
            Ok(status) => eprintln!("warning: git diff exited with status {} for '{}'", status, name),
            Err(e) => eprintln!("error: failed to run git diff for '{}': {}", name, e),
        }
    }
}
