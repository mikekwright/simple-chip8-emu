// Display module for the CHIP-8 emulator
//
// Module details:
//    - monochrome (black and white) display
//    - 64x32 pixel display
//    - (optional) SUPER-CHIP 128x64 pixel display

pub trait Display {
    fn width(&self) -> u16;
    fn height(&self) -> u16;

    fn clear(&mut self);
    fn read(&self, address: u16) -> u16;
    fn write(&mut self, address: u16, value: u16);
}

#[derive(Default, Debug, Clone)]
pub struct Chip8Display {
    pub width: u16,
    pub height: u16,
    pixels: Vec<u16>,
}

impl Display for Chip8Display {
    fn width(&self) -> u16 {
        self.width
    }
    fn height(&self) -> u16 {
        self.height
    }

    fn clear(&mut self) {
        self.pixels.fill(0);
    }

    fn read(&self, address: u16) -> u16 {
        self.pixels.get(address as usize).copied().unwrap_or(0)
    }

    fn write(&mut self, address: u16, value: u16) {
        if let Some(pixel) = self.pixels.get_mut(address as usize) {
            *pixel = value;
        }
    }
}

impl Chip8Display {
    pub fn new() -> Self {
        Self {
            width: 64,
            height: 32,
            pixels: vec![0; 64 * 32],
        }
    }

    /// Get a reference to the pixel buffer
    pub fn pixels(&self) -> &[u16] {
        &self.pixels
    }

    /// Set a pixel at the given coordinates
    pub fn set_pixel(&mut self, x: u16, y: u16, value: u16) {
        if x < self.width && y < self.height {
            let addr = y * self.width + x;
            self.write(addr, value);
        }
    }

    /// Get a pixel at the given coordinates
    pub fn get_pixel(&self, x: u16, y: u16) -> u16 {
        if x < self.width && y < self.height {
            let addr = y * self.width + x;
            self.read(addr)
        } else {
            0
        }
    }
}


//
// SuperChip8Display is a placeholder for the SUPER-CHIP display implementation
//

// pub struct SuperChip8Display {
//     pixels: Vec<u16>,
// }