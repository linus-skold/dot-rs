use crate::config::{dotrc_path, DotEntries, DotRc};

pub fn sync() {
    let dotrc = DotRc::load(&dotrc_path()).unwrap_or_else(|e| {
        eprintln!("error: failed to load ~/.dotrc: {}", e);
        std::process::exit(1);
    });

    let entries = DotEntries::load(&dotrc.entries_path()).unwrap_or_else(|e| {
        eprintln!("error: failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    let items = entries.get_entries();

    if items.is_empty() {
        println!("nothing to sync — no entries in {}", dotrc.entries_path().display());
        return;
    }

    for (name, source) in &items {
        if !source.exists() {
            eprintln!("warning: skipping '{}' — source does not exist: {}", name, source.display());
            continue;
        }

        let dest = dotrc.target.join(name);

        match super::copy_entry(source, &dest) {
            Ok(()) => println!("synced '{}': {} -> {}", name, source.display(), dest.display()),
            Err(e) => eprintln!("error: failed to sync '{}': {}", name, e),
        }
    }
}
