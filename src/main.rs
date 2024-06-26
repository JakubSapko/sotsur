#![no_std]
#![no_main]

use core::panic::PanicInfo;

// in default rust panic handler is a part of standard library which we can't use
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO_WORLD: &[u8] = b"HELLO WORLD!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // default entry point, linker looks for a _start function by default

    let vga_buff = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO_WORLD.iter().enumerate() {
        unsafe {
            *vga_buff.offset(i as isize * 2) = byte;
            *vga_buff.offset(i as isize * 2 + 1) = 0xb; // cyan
        }
    }
    loop {}
}
