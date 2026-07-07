use crate::config::{resolve_target, DotEntries, ENTRIES_FILENAME};
use crate::output::{error, info, warning};
use std::process::Command;

/// Shows differences between each tracked entry's copy in the dotfiles
/// folder and its live source location, using `git diff --no-index`.
pub fn diff() {
    let target = resolve_target();
    let entries_path = target.join(ENTRIES_FILENAME);

    let entries = DotEntries::load(&entries_path).unwrap_or_else(|e| {
        error!("failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    let mut items = entries.get_entries();
    items.sort_by(|a, b| a.0.cmp(&b.0));

    if items.is_empty() {
        info!("nothing to diff — no entries in {}", entries_path.display());
        return;
    }

    for (name, source) in &items {
        let tracked = target.join(name);

        if !tracked.exists() {
            warning!("skipping '{}' — not yet synced: {}", name, tracked.display());
            continue;
        }
        if !source.exists() {
            warning!("skipping '{}' — source does not exist: {}", name, source.display());
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
            Ok(status) => warning!("git diff exited with status {} for '{}'", status, name),
            Err(e) => error!("failed to run git diff for '{}': {}", name, e),
        }
    }
}
