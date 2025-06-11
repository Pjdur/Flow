#!/bin/bash

# This script moves the compiled executables to the bin directory
# This script assumes it is run from the root of this repository

# Create the bin directory if it doesn't exist
mkdir -p ./bin

# Copy Windows executable
cp /workspaces/Flow/target/x86_64-pc-windows-gnu/release/flow.exe ./bin/

# Copy Linux executable
cp /workspaces/Flow/target/x86_64-unknown-linux-gnu/release/flow ./bin/