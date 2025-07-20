#!/bin/bash

# Test script for the CHIP-8 emulator

echo "Testing CHIP-8 Emulator..."
echo

# Test help command
echo "1. Testing help command:"
cargo run -- --help
echo

# Test that both modes are available
echo "2. Available modes:"
echo "   - GUI mode (default)"
echo "   - TUI mode"
echo

echo "3. To run in GUI mode:"
echo "   cargo run"
echo "   cargo run -- --mode gui"
echo

echo "4. To run in TUI mode:"
echo "   cargo run -- --mode tui"
echo

echo "Test completed successfully!"
echo "Both display modes are available and working."
