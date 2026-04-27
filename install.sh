#!/bin/bash
set -e

REPO="Onion-L/codefart"
BIN_NAME="codefart"
INSTALL_DIR="/usr/local/bin"

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
    arm64|aarch64) TARGET="aarch64-apple-darwin" ;;
    x86_64)        TARGET="x86_64-apple-darwin" ;;
    *)
        echo "Error: unsupported architecture: $ARCH"
        echo "CodeFart currently supports macOS arm64 and x86_64."
        exit 1
        ;;
esac

# Get latest release URL
echo "Fetching latest release..."
RELEASE_URL=$(curl -sL "https://api.github.com/repos/$REPO/releases/latest" \
    | grep "browser_download_url.*$TARGET.tar.gz" \
    | cut -d '"' -f 4)

if [ -z "$RELEASE_URL" ]; then
    echo "Error: no release found for $TARGET"
    exit 1
fi

# Download and extract
echo "Downloading CodeFart..."
TMP_DIR=$(mktemp -d)
curl -sL "$RELEASE_URL" | tar xz -C "$TMP_DIR"

# Install
echo "Installing to $INSTALL_DIR/$BIN_NAME..."
if [ -w "$INSTALL_DIR" ]; then
    mv "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
else
    sudo mv "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
fi
chmod +x "$INSTALL_DIR/$BIN_NAME"

# Cleanup
rm -rf "$TMP_DIR"

echo ""
echo "✓ CodeFart installed successfully!"
echo ""
echo "To enable Claude notifications, run:"
echo "  codefart setup"
