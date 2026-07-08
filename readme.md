# dot-rs

A minimal, cross-platform config & dotfiles manager written in Rust.

---

## How it works

`dot` keeps a central **target folder** (your dotfiles repo) containing the actual config files, plus an `entries.toml` manifest that maps a short name to the original location the file/folder came from on your machine (per-OS, so the same manifest works on Windows and Unix). Commands copy files between that target folder and their real locations — nothing is symlinked.

The target folder is resolved in this order:

1. `DOTCONF` environment variable, if set
2. `~/.dotrc` file (a single line containing the path), if present
3. `~/.dot` as the default

## Install

```
cargo install --git https://github.com/linus-skold/dot-rs
```

This builds the `dot` binary.

## Getting started

```
dot init                       # create a local dotfiles folder + ~/.dotrc + entries.toml
dot init <git-url>              # or clone an existing dotfiles repo instead
```

`init` also runs `git init` (or `git clone`) in the target folder so you can version your dotfiles from the start.

## Commands

### `dot add <path> [--name <name>] [--raw]`

Copies a file or folder into the dotfiles folder and tracks it in `entries.toml`.

- `--name` sets the entry name (defaults to the file/folder name)
- `--raw` copies the file without adding it to `entries.toml` (useful for one-off files you don't want `apply`/`sync` to manage)

```
dot add ~/AppData/Roaming/nvim
dot add ~/.gitconfig --name gitconfig
```

### `dot apply [names...] [--all] [--force]`

Copies tracked entries **from** the dotfiles folder back **to** their original locations — i.e. installs your dotfiles onto the current machine.

- With no arguments, shows an interactive picker to choose which entries to apply
- Pass one or more `names` to apply specific entries without the picker
- `--all` applies every tracked entry without prompting
- `--force` overwrites target files even if they've changed locally (by default `apply` warns and skips when it detects local changes, so you don't accidentally clobber edits)

```
dot apply                 # interactive picker
dot apply nvim gitconfig  # apply just these entries
dot apply --all --force   # apply everything, overwrite local changes
```

### `dot sync`

The inverse of `apply` — copies each tracked entry **from** its source location back **into** the dotfiles folder, picking up any local edits you've made so they can be committed.

### `dot diff`

Shows differences between the tracked source files and the copies stored in the dotfiles folder, so you can see what `sync` would pull in before running it.

### `dot remove <name>`

Removes an entry from `entries.toml` (stops tracking it).

### `dot push`

Reserved for pushing the dotfiles repo upstream.

## Config files

- `~/.dotrc` — single line pointing at your dotfiles target folder
- `<target>/entries.toml` — maps entry name -> platform-specific source path, e.g.:

```toml
[nvim]
win = "~/AppData/Local/nvim"
unix = "~/.config/nvim"
```

Since both `win` and `unix` keys can live side by side, the same `entries.toml` (and dotfiles repo) can be shared across machines running different operating systems.
