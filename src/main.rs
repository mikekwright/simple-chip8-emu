mod hardware;
mod components;
mod tui;

use clap::{Parser, ValueEnum};

#[cfg(feature = "gui")]
mod simple_chip_emulator;
#[cfg(feature = "gui")]
use iced::{Settings, Application};
#[cfg(feature = "gui")]
use crate::simple_chip_emulator::SimpleChip8Emulator;

#[cfg(feature = "tui")]
use crate::tui::tui_display::TuiDisplay;

#[derive(Parser)]
#[command(name = "simple-chip8-emu")]
#[command(about = "A simple CHIP-8 emulator with GUI and TUI options")]
struct Cli {
    /// Display mode to use
    #[arg(short, long, value_enum, default_value_t = get_default_mode())]
    mode: DisplayMode,
}

#[derive(Clone, ValueEnum)]
enum DisplayMode {
    #[cfg(feature = "gui")]
    /// Use GUI mode with Iced
    Gui,
    #[cfg(feature = "tui")]
    /// Use TUI mode with Ratatui
    Tui,
}

fn get_default_mode() -> DisplayMode {
    #[cfg(all(feature = "tui"))]
    return DisplayMode::Tui;

    #[cfg(all(feature = "gui", not(feature = "tui")))]
    return DisplayMode::Gui;

    #[cfg(all(feature = "gui", feature = "tui"))]
    return DisplayMode::Tui;

    #[cfg(not(any(feature = "gui", feature = "tui")))]
    compile_error!("At least one display mode (gui or tui) must be enabled");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.mode {
        #[cfg(feature = "gui")]
        DisplayMode::Gui => {
            SimpleChip8Emulator::run(Settings::default())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        }
        #[cfg(feature = "tui")]
        DisplayMode::Tui => {
            TuiDisplay::new()?.run()?;
        }
    }

    Ok(())
}