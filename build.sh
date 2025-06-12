#!/bin/bash
set -e

# This script builds Flow for both Linux and Windows
# It assumes it is run from the root of this repository

# Build for Linux
echo "Building for Linux..."
cargo build --release --target x86_64-unknown-linux-gnu > /dev/null 2>&1

# Build for Windows
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu > /dev/null 2>&1

# Create the bin directory if it doesn't exist
mkdir -p ./bin

# Copy Linux executable
cp /workspaces/Flow/target/x86_64-unknown-linux-gnu/release/flow ./bin/

# Copy Windows executable
cp /workspaces/Flow/target/x86_64-pc-windows-gnu/release/flow.exe ./bin/

git commit -m "Build Flow for Linux and Windows"
echo "Built Flow for Linux and Windows and committed the binaries"
# Check if sha256sum is available

# Generate checksum for bin/flow.exe using sha256sum (only the hash value)
echo "Generating checksum for bin/flow.exe..."
CHECKSUM=$(sha256sum ./bin/flow.exe | awk '{ print $1 }')

# Save checksum to checksum.txt
echo "$CHECKSUM" > checksum.txt
echo "Checksum saved to checksum.txt"

# Commit the checksum.txt file
git add checksum.txt
git commit -m "Add checksum for bin/flow.exe"
echo "Committed checksum.txt"