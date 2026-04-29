use crate::config::{dotrc_path, DotRc};

pub fn sync() {
    let dotrc = DotRc::load(&dotrc_path()).unwrap_or_else(|e| {
        eprintln!("error: failed to load ~/.dotrc: {}", e);
        std::process::exit(1);
    });

    let target_base = dotrc.get_target().unwrap_or_else(|| {
        eprintln!("error: no target configured in ~/.dotrc. Add [settings] with target.win/unix.");
        std::process::exit(1);
    });

    let entries = dotrc.get_entries();

    if entries.is_empty() {
        println!("nothing to sync — no entries configured in ~/.dotrc");
        return;
    }

    for (name, source) in &entries {
        if !source.exists() {
            eprintln!(
                "warning: skipping '{}' — source path does not exist: {}",
                name,
                source.display()
            );
            continue;
        }

        let dest = target_base.join(name);

        match super::copy_dir_all(source, &dest) {
            Ok(()) => println!("synced '{}': {} -> {}", name, source.display(), dest.display()),
            Err(e) => eprintln!(
                "error: failed to sync '{}' ({} -> {}): {}",
                name,
                source.display(),
                dest.display(),
                e
            ),
        }
    }
}
