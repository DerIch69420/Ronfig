#!/usr/bin/env bash

set -e  # Exit on any error

BINARY_NAME="ronfig"
TARGET_DIR="$HOME/.local/bin"

echo "Building $BINARY_NAME..."
cargo build --release

mkdir -p "$TARGET_DIR"

echo "Installing $BINARY_NAME to $TARGET_DIR..."
cp "target/release/$BINARY_NAME" "$TARGET_DIR/"

if ! echo "$PATH" | grep -q "$TARGET_DIR"; then
    echo " $TARGET_DIR is not in your PATH."
    echo "Add this to your shell config file (e.g. ~/.bashrc, ~/.zshrc):"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
else
    echo "$BINARY_NAME is now available in your PATH."
fi

