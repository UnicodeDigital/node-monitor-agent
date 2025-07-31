#!/bin/bash

# Simple download script for monitor-agent Linux binary
# Downloads the latest release from GitHub

set -e

REPO="UnicodeDigital/node-monitor-agent"
BINARY_NAME="monitor-agent"
TARGET="x86_64-unknown-linux-gnu"

echo "🚀 Downloading latest monitor-agent for Linux x86_64..."

# Get latest release tag
echo "📡 Fetching latest release information..."
LATEST_TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    echo "❌ Failed to get latest release tag"
    exit 1
fi

echo "📦 Latest version: $LATEST_TAG"

# Construct download URL for Linux x86_64 tar.gz archive
ARCHIVE_NAME="${BINARY_NAME}-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${ARCHIVE_NAME}"

echo "⬇️  Downloading from: $DOWNLOAD_URL"

# Download and extract binary
if curl -L --progress-bar -o "$ARCHIVE_NAME" "$DOWNLOAD_URL"; then
    echo "✅ Downloaded successfully!"
    
    # Extract archive
    echo "📦 Extracting archive..."
    tar -xzf "$ARCHIVE_NAME"
    
    # Remove archive
    rm "$ARCHIVE_NAME"
    
    # Make executable
    chmod +x "$BINARY_NAME"
    echo "🔧 Made executable"
    
    # Show file info
    echo "📄 File info:"
    ls -lh "$BINARY_NAME"
    
    echo ""
    echo "🎉 Installation complete!"
    echo "💡 Run with: ./$BINARY_NAME --help"
    echo "💡 Test with: ./$BINARY_NAME --version"
else
    echo "❌ Download failed!"
    echo "💡 Please check if the release exists at: https://github.com/${REPO}/releases"
    echo "💡 Make sure you have an internet connection"
    exit 1
fi
