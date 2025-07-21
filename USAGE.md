# CHIP-8 Emulator Usage Guide

## Build Options

This emulator supports selective compilation using Cargo features to reduce binary size:

### Feature Flags

- `gui` - Includes GUI support (Iced, egui, eframe)
- `tui` - Includes TUI support (ratatui, crossterm)
- `default` - Both GUI and TUI features

### Helper Scripts

For convenience, several helper scripts are available in the `bin/` folder:

```bash
# View detailed build options and examples
bash bin/build_help.sh

# Test all build combinations
bash bin/verify_builds.sh

# Run comprehensive test suite
bash bin/test_all.sh
```

### Build Configurations

```bash
# Build with both GUI and TUI (default)
cargo build
cargo run

# Build TUI-only version (much smaller binary)
cargo build --no-default-features --features tui
cargo run --no-default-features --features tui

# Build GUI-only version
cargo build --no-default-features --features gui
cargo run --no-default-features --features gui

# Release builds for smaller, optimized binaries
cargo build --release --no-default-features --features tui
```

## Display Modes

This CHIP-8 emulator supports two display modes:

### GUI Mode (Default)

Uses Iced framework for a graphical user interface.

```bash
# Run with GUI (default)
cargo run

# Or explicitly specify GUI mode
cargo run -- --mode gui
```

### TUI Mode

Uses Ratatui and Crossterm for a terminal-based user interface.

```bash
# Run with TUI mode
cargo run -- --mode tui
```

## TUI Controls

When running in TUI mode, you can use the following controls:

- **Space**: Increment counter
- **T**: Toggle demo pixel patterns
- **R**: Reset display and counter
- **Q** or **Esc**: Quit the application

## Display Features

- **64x32 pixel monochrome display** (standard CHIP-8 resolution)
- **Real-time pixel rendering** in both GUI and TUI modes
- **Demo patterns** for testing display functionality
- **Counter tracking** for interaction testing

## Command Line Options

```bash
simple-chip8-emu [OPTIONS]

Options:
  -m, --mode <MODE>  Display mode to use [default: gui] [possible values: gui, tui]
  -h, --help         Print help
```

## Architecture

The emulator follows a modular design:

- `hardware/display.rs` - Core display logic and trait definitions
- `components/tui_display.rs` - TUI-specific display implementation
- `simple_chip_emulator.rs` - GUI-specific emulator implementation
- `main.rs` - CLI argument parsing and mode selection

The `Display` trait allows for easy extension to other display backends in the future.
