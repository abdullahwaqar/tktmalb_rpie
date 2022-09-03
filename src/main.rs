#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo, ptr::write_volatile};

// ? 3F20_0008 1<<3->8 turn pin21 to an output
// ? 3F20_001c gpio1_set 1<<21->8 turn pin21 on
// ? 3F20_0028 gpio1_clear 1<<21->8 turn pin21 off

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        write_volatile(0x3F20_0008 as *mut u32, 1 << 3);

        loop {
            write_volatile(0x3F20_001c as *mut u32, 1 << 21);

            for _ in 0..50000 {
                asm!("nop");
            }

            write_volatile(0x3F20_0028 as *mut u32, 1 << 21);

            for _ in 0..50000 {
                asm!("nop");
            }
        }
    }
}

/// * Handler for dealing with panic [hard_fault, soft_fault, bus_error,..]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
