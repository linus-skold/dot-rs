use crate::config::{dotrc_path, DotEntries, DotRc};

pub fn apply() {
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
        println!("nothing to apply — no entries in {}", dotrc.entries_path().display());
        return;
    }

    for (name, dest) in &items {
        let source = dotrc.target.join(name);

        if !source.exists() {
            eprintln!("warning: skipping '{}' — dotfiles source does not exist: {}", name, source.display());
            continue;
        }

        match super::copy_dir_all(&source, dest) {
            Ok(()) => println!("applied '{}': {} -> {}", name, source.display(), dest.display()),
            Err(e) => eprintln!("error: failed to apply '{}': {}", name, e),
        }
    }
}
