Simple Chip-8 EMU
==================================

A repository as I work on creating a chip-8 interpreter, because this project
sounds like fun and could be a great way to get into rust dev.

References
-----------------------------------

* [Chip8 Github](https://chip-8.github.io/links/)
* [Tutorial on Chip-8 Emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
* [Blog on Chip-8](https://otavio.dev/2024/12/08/chip-8-emulation/)
* [Chip-8 Instructions from Octo-ide](https://johnearnest.github.io/Octo/docs/chip8ref.pdf)
* [Octo a Chip-8 IDE](https://github.com/JohnEarnest/Octo)

Development Prerequisites
-----------------------------------

* [Rust](https://www.rust-lang.org/tools/install)
* [VS Code](https://code.visualstudio.com/)
* [VS Code Extension - ] [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Build

### Default Build (Both GUI and TUI)

```
cargo build
```

### Feature-Specific Builds

This project supports selective compilation to reduce binary size:

```bash
# TUI-only build (much smaller binary)
cargo build --no-default-features --features tui

# GUI-only build  
cargo build --no-default-features --features gui

# Release builds for optimized binaries
cargo build --release --no-default-features --features tui
```

**Benefits of TUI-only build:**

* Significantly smaller binary size
* Excludes all GUI dependencies (iced, egui, eframe)
* Perfect for server environments or minimal installations

## Run

### GUI Mode (Default)

```
cargo run
# or explicitly
cargo run -- --mode gui
```

### TUI Mode (Terminal UI)

```
cargo run -- --mode tui
```

The emulator supports both graphical (Iced) and terminal-based (Ratatui) display modes.
See [USAGE.md](USAGE.md) for detailed usage instructions and controls.

## Scripts

Helper scripts are located in the `bin/` folder:

```bash
# View all build options and commands
bash bin/build_help.sh

# Verify all build combinations work
bash bin/verify_builds.sh

# Run comprehensive test suite
bash bin/test_all.sh
```

## Debug in VS Code

1. Open this folder in VS Code.
2. Press `F5` or go to the Run & Debug panel and select `Debug executable`.

## Project Structure

* `src/` - Rust source files
* `bin/` - Helper scripts for building and testing
* `Cargo.toml` - Rust project manifest

## Resources

* [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)

---

*This project is a learning exercise and not intended for production use.*
