#!/bin/bash

# Build scripts for different feature combinations

echo "CHIP-8 Emulator Build Options"
echo "============================="
echo

echo "1. Build with both GUI and TUI (default):"
echo "   cargo build"
echo "   cargo run"
echo

echo "2. Build TUI-only version (smaller binary):"
echo "   cargo build --no-default-features --features tui"
echo "   cargo run --no-default-features --features tui"
echo

echo "3. Build GUI-only version:"
echo "   cargo build --no-default-features --features gui"
echo "   cargo run --no-default-features --features gui"
echo

echo "4. Release builds (optimized):"
echo "   cargo build --release --no-default-features --features tui"
echo "   cargo build --release --no-default-features --features gui"
echo "   cargo build --release  # both features"
echo

echo "Binary sizes will be significantly smaller when building TUI-only!"
echo "The TUI-only build excludes all GUI dependencies (iced, egui, eframe)."
