mod hardware;

use crate::hardware::display::{Chip8Display, Display};

fn main() {
    println!("Starting the program...");

    let mut display = Chip8Display::new();
    println!("Display initialized with {}x{} pixels.", display.width(), display.height());

    display.write(0x0001, 0xFFFF);
    println!("Checking the supplied value {}", display.read(0x0001));

    display.clear();
    println!("Display cleared. Pixel at (0, 0): {}", display.read(0x0000));

    println!("This is a simple Rust program to demonstrate basic functionality.");
}
