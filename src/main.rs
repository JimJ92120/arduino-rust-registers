#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    // PORT_B: 0x05
    // DDR_B: 0x04
    // PIN_13: 5 (in PORT_B)

    // 1 second delay with `nop`
    const DELAY: u32 = 1000000;

    unsafe {
        asm!{
            // set pin 13 input
            "sbi 0x04, 5",
            // set register `r20` to 0
            "clr r20",
        };

    }

    loop {
        unsafe {
            // on
            asm!{
                "sbi 0x05, 5"
            };

            for _ in 1..DELAY {
                asm!("nop");
            }

            // off
            asm!{
                "cbi 0x05, 5"
            };

            for _ in 1..DELAY {
                asm!("nop");
            }
        }
    }
}
