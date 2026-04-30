use crate::config::{resolve_target, DotEntries, ENTRIES_FILENAME};

pub fn sync() {
    let target = resolve_target();
    let entries_path = target.join(ENTRIES_FILENAME);

    let entries = DotEntries::load(&entries_path).unwrap_or_else(|e| {
        eprintln!("error: failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    let items = entries.get_entries();

    if items.is_empty() {
        println!("nothing to sync — no entries in {}", entries_path.display());
        return;
    }

    for (name, source) in &items {
        if !source.exists() {
            eprintln!("warning: skipping '{}' — source does not exist: {}", name, source.display());
            continue;
        }

        let dest = target.join(name);

        match super::copy_entry(source, &dest) {
            Ok(()) => println!("synced '{}': {} -> {}", name, source.display(), dest.display()),
            Err(e) => eprintln!("error: failed to sync '{}': {}", name, e),
        }
    }
}
