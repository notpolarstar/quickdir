#!/bin/bash
set -e

# Detect current shell and rc file
SHELL_NAME=$(basename "$SHELL")
case "$SHELL_NAME" in
    zsh)
        RC_FILE="$HOME/.zshrc"
        ;;
    bash)
        RC_FILE="$HOME/.bashrc"
        ;;
    fish)
        RC_FILE="$HOME/.config/fish/config.fish"
        ;;
    *)
        echo "Unsupported shell: $SHELL_NAME"
        exit 1
        ;;
esac

echo "Removing qd shell function from $RC_FILE..."
sed -i '/^qd()/,/^}/d' "$RC_FILE"

BIN_PATH="$(pwd)/target/release/quickdir"
if [ -f "$BIN_PATH" ]; then
    echo "Removing binary $BIN_PATH..."
    rm "$BIN_PATH"
fi

echo "Uninstall complete!"
