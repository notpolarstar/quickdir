#!/bin/bash
set -e

echo "Building quickdir..."
cargo build --release

BIN_PATH="$(pwd)/target/release/quickdir"

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

if grep -q '^qd()' "$RC_FILE"; then
    echo "Removing previous qd function from $RC_FILE..."
    sed -i '/^qd()/,/^}/d' "$RC_FILE"
fi

echo "Adding qd shell function to $RC_FILE..."
cat <<EOF >> "$RC_FILE"
qd() {
    "$BIN_PATH" "\$@"
    local dir
    dir=\$(< /tmp/quickdir_out)
    if [ -d "\$dir" ]; then
        cd "\$dir"
        echo > /tmp/quickdir_out
    fi
}
EOF

echo "Install complete!"
echo "Binary location: $BIN_PATH"
echo "Shell function added to $RC_FILE."
echo "Restart your shell or run 'source $RC_FILE' to enable the function."