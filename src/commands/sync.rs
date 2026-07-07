use crate::config::{resolve_target, DotEntries, ENTRIES_FILENAME};
use crate::output::{error, info, success, warning};

pub fn sync() {
    let target = resolve_target();
    let entries_path = target.join(ENTRIES_FILENAME);

    let entries = DotEntries::load(&entries_path).unwrap_or_else(|e| {
        error!("failed to load entries.toml: {}", e);
        std::process::exit(1);
    });

    let items = entries.get_entries();

    if items.is_empty() {
        info!("nothing to sync — no entries in {}", entries_path.display());
        return;
    }

    for (name, source) in &items {
        if !source.exists() {
            warning!("skipping '{}' — source does not exist: {}", name, source.display());
            continue;
        }

        let dest = target.join(name);

        match super::copy_entry(source, &dest) {
            Ok(()) => success!("synced '{}': {} -> {}", name, source.display(), dest.display()),
            Err(e) => error!("failed to sync '{}': {}", name, e),
        }
    }
}
