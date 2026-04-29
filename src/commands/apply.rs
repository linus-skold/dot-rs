use crate::config::{dotrc_path, DotRc};

pub fn apply() {
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
        println!("nothing to apply — no entries configured in ~/.dotrc");
        return;
    }

    for (name, dest) in &entries {
        let source = target_base.join(name);

        if !source.exists() {
            eprintln!(
                "warning: skipping '{}' — dotfiles source does not exist: {}",
                name,
                source.display()
            );
            continue;
        }

        match super::copy_dir_all(&source, dest) {
            Ok(()) => println!("applied '{}': {} -> {}", name, source.display(), dest.display()),
            Err(e) => eprintln!(
                "error: failed to apply '{}' ({} -> {}): {}",
                name,
                source.display(),
                dest.display(),
                e
            ),
        }
    }
}
