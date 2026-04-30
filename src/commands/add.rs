use crate::config::{dotrc_path, DotEntries, DotRc, expand_tilde};

pub fn add(path: &str, name: Option<&str>, raw: bool) {
    let source = expand_tilde(path);

    if !source.exists() {
        eprintln!("error: path does not exist: {}", source.display());
        std::process::exit(1);
    }

    let entry_name = name
        .map(|n| n.to_string())
        .or_else(|| {
            source.file_name().and_then(|n| n.to_str()).map(|n| n.to_string())
        })
        .unwrap_or_else(|| {
            eprintln!("error: could not determine entry name from path");
            std::process::exit(1);
        });

    let dotrc = DotRc::load(&dotrc_path()).unwrap_or_else(|e| {
        eprintln!("error: failed to load ~/.dotrc: {}", e);
        std::process::exit(1);
    });

    let dest = dotrc.target.join(&entry_name);

    if let Err(e) = super::copy_entry(&source, &dest) {
        eprintln!("error: failed to copy '{}' to '{}': {}", source.display(), dest.display(), e);
        std::process::exit(1);
    }

    if raw {
        println!("copied '{}' -> {} (raw, not tracked)", entry_name, dest.display());
        return;
    }

    let mut entries = DotEntries::load(&dotrc.entries_path()).unwrap_or_else(|e| {
        eprintln!("error: failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    if entries.is_tracked(&entry_name) {
        eprintln!("warning: '{}' is already tracked in entries.toml — skipping", entry_name);
        return;
    }

    // Store the original source path (unexpanded) so it stays portable.
    entries.add_entry(&entry_name, path);

    if let Err(e) = entries.save() {
        eprintln!("error: failed to save entries.toml: {}", e);
        std::process::exit(1);
    }

    println!("tracked '{}': {} -> {}", entry_name, source.display(), dest.display());
}
