#!/bin/sh

# Exit if any command fails
set -eu

# Ensure script runs from the directory it resides in
cd "$(dirname "$0")"

# Define variables
LINUX_TARGET="x86_64-unknown-linux-gnu"
WINDOWS_TARGET="x86_64-pc-windows-gnu"
BIN_DIR="./bin"

echo "Building for Linux..."
cargo build --release --target "$LINUX_TARGET" >/dev/null 2>&1

echo "Building for Windows..."
cargo build --release --target "$WINDOWS_TARGET" >/dev/null 2>&1

# Create bin directory
mkdir -p "$BIN_DIR"

# Copy executables, exit if missing
if [ -f "target/$LINUX_TARGET/release/flow" ]; then
    cp "target/$LINUX_TARGET/release/flow" "$BIN_DIR/"
else
    echo "Linux build missing. Aborting."
    exit 1
fi

if [ -f "target/$WINDOWS_TARGET/release/flow.exe" ]; then
    cp "target/$WINDOWS_TARGET/release/flow.exe" "$BIN_DIR/"
else
    echo "Windows build missing. Aborting."
    exit 1
fi

# Git commit for binaries
git add "$BIN_DIR/flow" "$BIN_DIR/flow.exe"
if ! git commit -m "Build Flow for Linux and Windows"; then
    echo "No changes to commit for binaries."
fi

# Generate checksums
echo "Generating checksum for bin/flow.exe..."
WIN_CHECK=$(sha256sum "$BIN_DIR/flow.exe" | awk '{ print $1 }')

echo "Generating checksum for bin/flow..."
LIN_CHECK=$(sha256sum "$BIN_DIR/flow" | awk '{ print $1 }')

# Save checksums
echo "$WIN_CHECK" > win-check.txt
echo "$LIN_CHECK" > linux-check.txt

echo "Checksums saved."

# Git commit for checksums
git add win-check.txt linux-check.txt
if ! git commit -m "Add checksums for bin/flow and bin/flow.exe"; then
    echo "No changes to commit for checksums."
fi

echo "Build and checksum generation completed successfully."
