mod simple_chip_emulator;
mod hardware;

use iced::{Settings, Application};
use crate::simple_chip_emulator::SimpleChip8Emulator;

fn main() -> iced::Result {
    SimpleChip8Emulator::run(Settings::default())
}