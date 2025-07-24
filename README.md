# quickdir

quickdir is a fast terminal-based directory picker for Linux shells. It lets you visually browse directories in a TUI and quickly jump to your selection in your shell.

## Features
- Fast keyboard navigation
- Three-pane view: previous, current, next directories
- Shell integration for instant `cd` after selection

## Installation
Run the install script:

```sh
./install.sh
```

This will build the binary and add the shell function (`qd`) to your shell rc file (e.g., `.zshrc`, `.bashrc`).

Currently, the compatible shells are `zsh`, `bash` and `fish`.

## Usage
Restart your shell or run `source ~/.zshrc` (or your shell's rc file).

Then use:

```sh
qd
```

Navigate with arrow keys, select a directory with SPACE. Your shell will change to the selected directory automatically.

## Requirements
- Linux
- Rust toolchain (for building)

## Uninstall
Run the uninstall script:

```sh
./uninstall.sh
```

This will delete the binary and remove the shell function from your shell's rc file.
