use core::ptr::write_volatile;

const SCREEN_ADDR: usize = 0x57C0;
// Width of the screen (in tiles)
const SCREEN_WIDTH: usize = 8;
// Height of the screen (in tiles)
const SCREEN_HEIGHT: usize = 8;

// Width of the screen (in pixels)
pub const TOTAL_WIDTH: usize = Tile::TILE_WIDTH * SCREEN_WIDTH;
// Height of the screen (in pixels)
pub const TOTAL_HEIGHT: usize = Tile::TILE_HEIGHT * SCREEN_HEIGHT;

#[allow(unused)]
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

#[allow(unused)]
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

    pub fn set_tile(&mut self, tile_index: u8, tile: &Tile, color: u32) {
        unsafe {
            write_volatile(self.tile, tile_index);
            write_volatile(self.pixel, tile.mask);
            write_volatile(self.color, color);
            write_volatile(self.display, 1);
        }
    }
}

/// A TILE_WIDTH x TILE_HEIGHT chunk of screen, in which a group a pixel of the same color can be added and then sent to the screen all at once
#[derive(Default)]
pub struct Tile {
    mask: u64,
}

#[allow(unused)]
impl Tile {
    /// Width of one tile (in pixels)
    pub const TILE_WIDTH: usize = 8;
    /// Height of one tile (in pixels)
    pub const TILE_HEIGHT: usize = 6;

    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a tile with only the tile pixel (x,y) added
    /// the x and y arguments are relative to the tile, so they must be inferior to TILE_WIDTH and TILE_HEIGHT respectively
    pub fn from_pixel(x: usize, y: usize) -> Self {
        let mut tile = Self { mask: 0 };
        tile.add_pixel(x, y);
        tile
    }

    /// Adds the (x,y) pixel in the tile
    /// the x and y arguments are relative to the tile, so they must be inferior to TILE_WIDTH and TILE_HEIGHT respectively
    pub fn add_pixel(&mut self, x: usize, y: usize) {
        self.mask |= (1u64 << (Self::TILE_WIDTH - x - 1)) << (Self::TILE_WIDTH * (y + 1));
    }

    /// Returns the byte describing the tile at position (x,y) in the tile grid
    /// Because the screen is 8x8 tiles, the row and col index of the tageted tile can be coded on 3 bits (the first 3 are for the x-coord and the next 3 are for the y-coord)
    pub fn tile_pos_to_index(x: usize, y: usize) -> u8 {
        return ((x & 0b00000111) | ((y & 0b00000111) << 3)) as u8;
    }

    /// Returns the byte describing the tile in which the screen point (x,y) is
    /// Because the screen is 8x8 tiles, the row and col index of the tageted tile can be coded on 3 bits (the first 3 are for the x-coord and the next 3 are for the y-coord)
    pub fn screen_pixel_to_index(x: usize, y: usize) -> u8 {
        return Self::tile_pos_to_index(x / Self::TILE_WIDTH, y / Self::TILE_HEIGHT);
    }
}

pub fn get_color(r: u8, g: u8, b: u8) -> u32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    ((r << 16) | (g << 8) | b) << 8
}
