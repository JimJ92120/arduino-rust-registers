#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::panic::PanicInfo;

mod helpers;
mod ports;
mod serial;

use ports::{ Port, PortB };
use serial::Serial;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {   
    const DELAY_DURATION: u32 = 1000000;

    Port::set_output::<PortB>(PortB::PIN_13);

    Serial::set_baud_rate(57600);
    Serial::enable_transmissitter();

    loop {
        Serial::write_character('A');
        Serial::write_character('\n');

        Port::set_pin_high::<PortB>(PortB::PIN_13);
        helpers::delay(DELAY_DURATION);

        Port::set_pin_low::<PortB>(PortB::PIN_13);
        helpers::delay(DELAY_DURATION);
    }
}
