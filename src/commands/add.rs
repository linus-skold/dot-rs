use crate::config::{collapse_home, expand_tilde, resolve_target, DotEntries, ENTRIES_FILENAME};
use crate::output::{error, success, warning};

pub fn add(path: &str, name: Option<&str>, raw: bool) {
    let source = expand_tilde(path);

    if !source.exists() {
        error!("path does not exist: {}", source.display());
        std::process::exit(1);
    }

    let entry_name = name
        .map(|n| n.to_string())
        .or_else(|| {
            source.file_name().and_then(|n| n.to_str()).map(|n| n.to_string())
        })
        .unwrap_or_else(|| {
            error!("could not determine entry name from path");
            std::process::exit(1);
        });

    let target = resolve_target();
    let dest = target.join(&entry_name);

    if let Err(e) = super::copy_entry(&source, &dest) {
        error!("failed to copy '{}' to '{}': {}", source.display(), dest.display(), e);
        std::process::exit(1);
    }

    if raw {
        success!("copied '{}' -> {} (raw, not tracked)", entry_name, dest.display());
        return;
    }

    let entries_path = target.join(ENTRIES_FILENAME);
    let mut entries = DotEntries::load(&entries_path).unwrap_or_else(|e| {
        error!("failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    if entries.is_tracked(&entry_name) {
        warning!("'{}' is already tracked in entries.toml — skipping", entry_name);
        return;
    }

    // Collapse home dir to ~/ so the stored path is portable across usernames.
    let portable_path = collapse_home(&source);
    entries.add_entry(&entry_name, &portable_path);

    if let Err(e) = entries.save() {
        error!("failed to save entries.toml: {}", e);
        std::process::exit(1);
    }

    success!("tracked '{}': {} -> {}", entry_name, source.display(), dest.display());
}
