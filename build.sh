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

git add ./bin/flow ./bin/flow.exe
git commit -m "Build Flow for Linux and Windows"
echo "Built Flow for Linux and Windows and committed the binaries"
# Check if sha256sum is available

# Generate checksum for bin/flow.exe using sha256sum (only the hash value)
echo "Generating checksum for bin/flow.exe..."
WIN_CHECK=$(sha256sum ./bin/flow.exe | awk '{ print $1 }')

echo "Generating checksum for bin/flow..."
LIN_CHECK=$(sha256sum ./bin/flow | awk '{ print $1 }')

# Save Windows checksum to win-check.txt
echo "$WIN_CHECK" > win-check.txt
echo "Windows Checksum saved to win-check.txt"

# Save Linux checksum to linux-check.txt
echo "$LIN_CHECK" > linux-check.txt
echo "Linux Checksum saved to linux-check.txt"

# Commit the win-check.txt and linux-check.txt files
git add win-check.txt linux-check.txt
git commit -m "Add checksums for bin/flow and bin/flow.exe"
echo "Committed win-check.txt and linux-check.txt with checksums"