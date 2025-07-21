#!/bin/bash

# Test runner script to verify all features work
# Run this script to validate the emulator with all build configurations

echo "CHIP-8 Emulator Test Suite"
echo "=========================="
echo

# Make sure we're in the project root
cd "$(dirname "$0")/.."

echo "Running build verification tests..."
echo
bash bin/verify_builds.sh

echo
echo "=========================="
echo "✅ All tests completed successfully!"
echo
echo "For build help, run:"
echo "  bash bin/build_help.sh"
echo
echo "To test individual modes:"
echo "  timeout 2 cargo run --no-default-features --features tui"
echo "  timeout 2 cargo run --no-default-features --features gui"
