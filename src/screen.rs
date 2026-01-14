use core::ptr::write_volatile;

const SCREEN_ADDR: usize = 0x57C0;
// Width of one tile (in pixels)
const TILE_WIDTH: usize = 8;
// Height of one tile (in pixels)
const TILE_HEIGHT: usize = 6;
// Width of the screen (in tiles)
const SCREEN_WIDTH: usize = 8;
// Height of the screen (in tiles)
const SCREEN_HEIGHT: usize = 8;

// Width of the screen (in pixels)
pub const TOTAL_WIDTH: usize = TILE_WIDTH * SCREEN_WIDTH;
// Height of the screen (in pixels)
pub const TOTAL_HEIGHT: usize = TILE_HEIGHT * SCREEN_HEIGHT;

pub struct Screen {
    /// Which pixels in the tile to set
    pixel: *mut u64,
    /// The color of the pixels to set
    color: *mut u32,
    /// Which tile
    tile: *mut u8,
    /// Flag to set to 1 to send the changes to the screen
    display: *mut u8,
}

impl Screen {
    pub fn new() -> Self {
        let ptr = SCREEN_ADDR as *mut u8;

        unsafe {
            Self {
                pixel: ptr.offset(0) as *mut u64,
                color: ptr.offset(8) as *mut u32,
                tile: ptr.offset(12) as *mut u8,
                display: ptr.offset(13) as *mut u8,
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        unsafe {
            if x >= TOTAL_WIDTH || y >= TOTAL_HEIGHT {
                return;
            }

            let tile = get_tile(x, y);

            let pixel_mask = get_pixel_mask(x % TILE_WIDTH, y % TILE_HEIGHT);

            write_volatile(self.tile, tile);
            write_volatile(self.pixel, pixel_mask);
            write_volatile(self.color, color);
            write_volatile(self.display, 1);
        }
    }
}

/// Returns the byte describing the tile in which the point (x,y) is
/// Because the screen is 8x8 tiles, the row and col index of the tageted tile can be coded on 3 bits (the first 3 are for the x-coord and the next 3 are for the y-coord)
fn get_tile(x: usize, y: usize) -> u8 {
    return (((x / TILE_WIDTH) & 0b00000111) | (((y / TILE_HEIGHT) & 0b00000111) << 3)) as u8;
}

/// Returns 8 bytes where there is the bit corresponding to point (x,y) to 1
/// the x and y arguments are relative to the tile, so they must be inferior to TILE_WIDTH and TILE_HEIGHT respectively
fn get_pixel_mask(x: usize, y: usize) -> u64 {
    return (1u64 << (TILE_WIDTH - x - 1)) << (TILE_WIDTH * (y + 1));
}

pub fn get_color(r: u8, g: u8, b: u8) -> u32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    ((r << 16) | (g << 8) | b) << 8
}
