mod simple_chip_emulator;
mod hardware;
mod components;

use clap::{Parser, ValueEnum};
use iced::{Settings, Application};
use crate::simple_chip_emulator::SimpleChip8Emulator;
use crate::components::tui_display::TuiDisplay;

#[derive(Parser)]
#[command(name = "simple-chip8-emu")]
#[command(about = "A simple CHIP-8 emulator with GUI and TUI options")]
struct Cli {
    /// Display mode to use
    #[arg(short, long, value_enum, default_value_t = DisplayMode::Gui)]
    mode: DisplayMode,
}

#[derive(Clone, ValueEnum)]
enum DisplayMode {
    /// Use GUI mode with Iced
    Gui,
    /// Use TUI mode with Ratatui
    Tui,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.mode {
        DisplayMode::Gui => {
            SimpleChip8Emulator::run(Settings::default())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        }
        DisplayMode::Tui => {
            let mut tui_display = TuiDisplay::new()?;
            tui_display.run()?;
        }
    }

    Ok(())
}