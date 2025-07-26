const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 24;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

const SCREEN_ADDR: usize = 0x5000;
// const SCREEN_ADDR: *mut [u8; SCREEN_SIZE] = 0x5000 as *mut [u8; SCREEN_SIZE];
pub struct Console {
    pixels: &'static mut [u8; SCREEN_SIZE],
}

impl Console {
    pub fn new() -> Self {
        let pixels = unsafe { &mut *(SCREEN_ADDR as *mut [u8; SCREEN_SIZE]) };
        Self { pixels }
    }

    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|p| *p = 0);
    }
    pub fn write_char(&mut self, index: usize, c: char) {
        self.pixels[index] = c.to_ascii_lowercase() as u8;
    }

    pub fn write_string(&mut self, index: usize, string: &[u8]) {
        let len = string.len();
        for i in 0..len {
            self.pixels[index + i] = string[i];
        }
    }

    pub fn write_number(&mut self, index: usize, mut number: u32) {
        let mut digits = [0; 10];
        let mut i = 0;
        while number > 0 && i < 10 {
            digits[i] = (number % 10) as u8;
            i = i + 1;
            number = number / 10;
        }
        for j in 0..i {
            self.pixels[index + j] = '0' as u8 + digits[i - j - 1];
        }
    }
}
