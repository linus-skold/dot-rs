use crate::config::{resolve_target, DotEntries, ENTRIES_FILENAME};
use crate::output::{error, success, warning};

pub fn remove(name: &str) {
    let target = resolve_target();
    let entries_path = target.join(ENTRIES_FILENAME);

    let mut entries = DotEntries::load(&entries_path).unwrap_or_else(|e| {
        error!("failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    if !entries.remove_entry(name) {
        error!("no entry named '{}' in {}", name, entries_path.display());
        std::process::exit(1);
    }

    if let Err(e) = entries.save() {
        error!("failed to save entries.toml: {}", e);
        std::process::exit(1);
    }

    let copy = target.join(name);
    if copy.exists() {
        let result = if copy.is_dir() {
            std::fs::remove_dir_all(&copy)
        } else {
            std::fs::remove_file(&copy)
        };
        if let Err(e) = result {
            warning!("untracked '{}' but failed to remove {}: {}", name, copy.display(), e);
            return;
        }
    }

    success!("removed '{}'", name);
}
