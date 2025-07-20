# CHIP-8 Emulator Usage Guide

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
