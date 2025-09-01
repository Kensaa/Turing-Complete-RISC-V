use core::ptr::{read_volatile, write_volatile};

const KEYBOARD_ADDRESS: usize = 0x5780;
pub struct Keyboard {
    read: *mut u8,
    read_available: *const u8,
    key_val: *const u8,
    key_up: *const u8,
}

impl Keyboard {
    pub fn new() -> Self {
        let ptr = KEYBOARD_ADDRESS as *mut u8;
        unsafe {
            let read = ptr.offset(0) as *mut u8;
            let read_available = ptr.offset(1);
            let key_val = ptr.offset(2);
            let key_up = ptr.offset(3);

            Keyboard {
                read_available,
                read,
                key_val,
                key_up,
            }
        }
    }

    /// Wait for a key, returns (key,is_key_up)
    pub fn read_key(&mut self) -> (char, bool) {
        unsafe {
            loop {
                if read_volatile(self.read_available) != 0 {
                    break;
                }
            }

            write_volatile(self.read, 1);
            let key = read_volatile(self.key_val);
            let keyup = if read_volatile(self.key_up) == 0 {
                false
            } else {
                true
            };
            return (key as char, keyup);
        }
    }
}
