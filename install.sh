#!/bin/bash
set -e

REPO="Onion-L/codefart"
BIN_NAME="codefart"
INSTALL_DIR="${CODEFART_INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Darwin)
        case "$ARCH" in
            arm64|aarch64) TARGET="aarch64-apple-darwin" ;;
            x86_64)        TARGET="x86_64-apple-darwin" ;;
            *)
                echo "Error: unsupported architecture on macOS: $ARCH"
                exit 1
                ;;
        esac
        ;;
    Linux)
        case "$ARCH" in
            arm64|aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
            x86_64)        TARGET="x86_64-unknown-linux-gnu" ;;
            *)
                echo "Error: unsupported architecture on Linux: $ARCH"
                exit 1
                ;;
        esac
        ;;
    *)
        echo "Error: unsupported OS: $OS"
        echo "CodeFart currently supports macOS and Linux."
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
mkdir -p "$INSTALL_DIR"
echo "Installing to $INSTALL_DIR/$BIN_NAME..."
mv "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

# Cleanup
rm -rf "$TMP_DIR"

echo ""
echo "✓ CodeFart installed successfully!"
echo ""
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        echo "Add CodeFart to your PATH:"
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
        echo ""
        echo "For zsh:"
        echo "  echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.zshrc"
        echo ""
        echo "For bash:"
        echo "  echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.bashrc"
        echo ""
        ;;
esac
echo "To enable Claude notifications, run:"
echo "  codefart setup"
