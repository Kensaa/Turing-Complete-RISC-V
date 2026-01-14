const CONSOLE_WIDTH: usize = 80;
const CONSOLE_HEIGHT: usize = 24;
const CONSOLE_SIZE: usize = CONSOLE_WIDTH * CONSOLE_HEIGHT;
const MAX_DIGIT: usize = 2u128.pow(64).ilog10() as usize;

const CONSOLE_ADDR: usize = 0x5000;
pub struct Console {
    pixels: &'static mut [u8; CONSOLE_SIZE],
}

impl Console {
    pub fn new() -> Self {
        let pixels = unsafe { &mut *(CONSOLE_ADDR as *mut [u8; CONSOLE_SIZE]) };
        Self { pixels }
    }

    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|p| *p = 0);
    }
    pub fn write_char(&mut self, index: usize, c: char) {
        self.pixels[index] = c.to_ascii_lowercase() as u8;
    }

    pub fn write_string(&mut self, index: usize, string: &[u8]) -> usize {
        let len = string.len();
        for i in 0..len {
            self.pixels[index + i] = string[i];
        }
        return len;
    }

    pub fn write_number(&mut self, index: usize, mut number: u64) -> usize {
        if number == 0 {
            self.pixels[index] = '0' as u8;
            return 1;
        }

        let mut digits = [0; MAX_DIGIT];
        let mut i = 0;
        while number > 0 && i < MAX_DIGIT {
            digits[i] = (number % 10) as u8;
            i = i + 1;
            number = number / 10;
        }
        for j in 0..i {
            self.pixels[index + j] = '0' as u8 + digits[i - j - 1];
        }

        return i;
    }
}
