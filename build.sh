#!/bin/bash
set -euo pipefail

# Ensure script runs from the repository root
cd "$(dirname "$0")"

# Output directories
LINUX_TARGET="x86_64-unknown-linux-gnu"
WINDOWS_TARGET="x86_64-pc-windows-gnu"
BIN_DIR="./bin"

echo "Building for Linux..."
cargo build --release --target "$LINUX_TARGET" > /dev/null 2>&1

echo "Building for Windows..."
cargo build --release --target "$WINDOWS_TARGET" > /dev/null 2>&1

mkdir -p "$BIN_DIR"

# Copy executables to bin directory
cp "target/$LINUX_TARGET/release/flow" "$BIN_DIR/" || { echo "Linux build failed or missing."; exit 1; }
cp "target/$WINDOWS_TARGET/release/flow.exe" "$BIN_DIR/" || { echo "Windows build failed or missing."; exit 1; }

git add "$BIN_DIR/flow" "$BIN_DIR/flow.exe"
git commit -m "Build Flow for Linux and Windows" || echo "No changes to commit for binaries."

# Generate checksums
echo "Generating checksum for bin/flow.exe..."
WIN_CHECK=$(sha256sum "$BIN_DIR/flow.exe" | awk '{ print $1 }')

echo "Generating checksum for bin/flow..."
LIN_CHECK=$(sha256sum "$BIN_DIR/flow" | awk '{ print $1 }')

# Save checksums
echo "$WIN_CHECK" > win-check.txt
echo "$LIN_CHECK" > linux-check.txt
echo "Checksums saved."

git add win-check.txt linux-check.txt
git commit -m "Add checksums for bin/flow and bin/flow.exe" || echo "No changes to commit for checksums."

echo "Build and checksum generation completed successfully."
