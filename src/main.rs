#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
extern crate alloc;

use crate::console::Console;
use crate::keyboard::Keyboard;
use core::alloc::{GlobalAlloc, Layout};
use core::arch::{asm, global_asm};
use core::cell::UnsafeCell;
use core::panic::PanicInfo;
use core::ptr::{self, addr_of, write_volatile};

mod console;
mod keyboard;

global_asm!(include_str!("init.S"));
unsafe extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section
    static _sidata: u32;

    static mut _heap_start: u8;
    static mut _heap_end: u8;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("ebreak");
    }
    loop {}
}

struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: UnsafeCell<usize>,
}
unsafe impl Sync for BumpAllocator {}
unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();

        // load current pointer
        let current = *self.next.get();

        // align up
        let aligned = (current + align - 1) & !(align - 1);
        let new_next = aligned.checked_add(size).unwrap_or(usize::MAX);

        let test = 0x5000 as *mut u32;
        write_volatile(test, aligned as u32);
        if new_next > self.heap_end {
            write_volatile(test.offset(4), new_next as u32);
            write_volatile(test.offset(5), self.heap_start as u32);
            write_volatile(test.offset(6), self.heap_end as u32);
            ptr::null_mut()
        } else {
            *self.next.get() = new_next;
            aligned as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // no-op for bump allocator
    }
}

// Provide a global instance (initialized later)
#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator {
    heap_start: 0,
    heap_end: 0,
    next: UnsafeCell::new(0),
};

#[unsafe(export_name = "_rust_start")]
pub unsafe extern "C" fn start() {
    unsafe {
        r0::zero_bss(&raw mut _sbss, &raw mut _ebss);
        // r0::init_data(&raw mut _sdata, &raw mut _edata, &_sidata);

        // Init heap allocator
        let s = addr_of!(_heap_start) as usize;
        let e = addr_of!(_heap_end) as usize;

        // initialize the static with real addresses (uses pointer write because ALLOCATOR is 'static')
        let allocator_ptr = &ALLOCATOR as *const _ as *mut BumpAllocator;
        (*allocator_ptr).heap_start = s;
        (*allocator_ptr).heap_end = e;
        *(*allocator_ptr).next.get() = s;

        main();
    }
}

unsafe fn halt() -> () {
    asm!("ebreak");
}

unsafe fn main() {
    // let ptr = 0x5000 as *mut u8;
    // let randptr = 0xA008 as *mut u16;

    // *ptr = (*ptr2 % *ptr3) as u8;
    // *ptr = 'z'.to_ascii_lowercase() as u8;
    let mut console = Console::new();
    let mut keyboard = Keyboard::new();

    let mut index = 0;
    loop {
        let (key, keyup) = keyboard.read_key();
        if keyup {
            console.write_char(index, key);
            index += 1;
        }
    }
    // console.write_number(0, fibo2(black_box(30)));
    // }
    // for i in 0..15 {
    //     console.write_number(i * 16, fibo2(i as u8));
    // }
    // let n = black_box(m(12, 10));
    // *ptr = n;
    // console.write_number(0, black_box(fibo2(30)));
    // console.write_number(0, black_box(a(35, 8) as u32));
    // for i in 0..15 {
    //     console.write_number(i * 16, black_box(fibo2(i as u8) as u32));
    // }

    halt();
}

fn a(a: u8, b: u8) -> u8 {
    a % b
}

fn fibo(n: u8) -> u16 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return fibo(n - 1) + fibo(n - 2);
}

fn fibo2(n: u8) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    return b;
}
