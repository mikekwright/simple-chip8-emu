#!/bin/bash

echo "Testing all build combinations for CHIP-8 emulator..."
echo "====================================================="

echo "1. Testing TUI-only build (default) ..."
if cargo check --quiet; then
    echo "   ✅ TUI-only build: SUCCESS"
else
    echo "   ❌ TUI-only build: FAILED"
    exit 1
fi

echo "2. Testing GUI-only build..."
if cargo check --no-default-features --features gui --quiet; then
    echo "   ✅ GUI-only build: SUCCESS"
else
    echo "   ❌ GUI-only build: FAILED"
    exit 1
fi

echo "3. Testing both features ..."
if cargo check --features gui --quiet; then
    echo "   ✅ Default build (both features): SUCCESS"
else
    echo "   ❌ Default build: FAILED"
    exit 1
fi

echo
echo "✅ All build combinations work correctly!"
echo
echo "Available commands:"
echo "  cargo run                                        # TUI-only (default)"
echo "  cargo run --no-default-features --features gui   # GUI-only"
echo "  cargo run --features gui                         # Both"
