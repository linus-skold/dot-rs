use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::config::{expand_tilde, DotRc, DOTRC_FILENAME};

pub fn init(url: &str, path: Option<&str>) {
    let dest = resolve_dest(url, path);

    if dest.exists() {
        eprintln!(
            "error: destination '{}' already exists",
            dest.display()
        );
        std::process::exit(1);
    }

    println!("cloning {} -> {}", url, dest.display());
    git_clone(url, &dest);

    // Create ~/.dot/ (the default target directory)
    let dot_home = expand_tilde("~/.dot");
    if !dot_home.exists() {
        fs::create_dir_all(&dot_home).unwrap_or_else(|e| {
            eprintln!("error: failed to create {}: {}", dot_home.display(), e);
            std::process::exit(1);
        });
        println!("created {}", dot_home.display());
    }

    // Write .dotrc into the cloned repo if it doesn't already have one.
    let dotrc_path = dest.join(DOTRC_FILENAME);
    if dotrc_path.exists() {
        println!(".dotrc already present — skipping creation");
    } else {
        let dotrc = DotRc::new_default(&dotrc_path);
        dotrc.save().unwrap_or_else(|e| {
            eprintln!("error: failed to write {}: {}", dotrc_path.display(), e);
            std::process::exit(1);
        });
        println!("created {}", dotrc_path.display());
    }

    println!("done — dotfiles repo ready at {}", dest.display());
}

fn resolve_dest(url: &str, path: Option<&str>) -> PathBuf {
    if let Some(p) = path {
        return expand_tilde(p);
    }
    // Derive repo name from URL: strip trailing `.git`, take last path segment.
    let name = url
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("dotfiles")
        .trim_end_matches(".git");
    std::env::current_dir()
        .expect("failed to get current dir")
        .join(name)
}

fn git_clone(url: &str, dest: &PathBuf) {
    let status = Command::new("git")
        .args(["clone", url, dest.to_str().expect("non-UTF-8 path")])
        .status()
        .unwrap_or_else(|e| {
            eprintln!("error: failed to run git: {}", e);
            std::process::exit(1);
        });

    if !status.success() {
        eprintln!("error: git clone failed");
        std::process::exit(1);
    }
}
