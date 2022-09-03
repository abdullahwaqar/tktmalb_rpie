#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// * Handler for dealing with panic [hard_fault, soft_fault, bus_error,..]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
