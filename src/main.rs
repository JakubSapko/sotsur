#![no_std]
#![no_main]

use core::panic::PanicInfo;

// in default rust panic handler is a part of standard library which we can't use
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // default entry point, linker looks for a _start function by default
    loop {}
}
