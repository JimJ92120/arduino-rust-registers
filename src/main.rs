#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::panic::PanicInfo;

mod helpers;
mod macros;
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

    let port_b = PortB {};

    port_b.set_output(PortB::PIN_13);
    port_b.set_output(PortB::PIN_8);

    Serial::set_baud_rate(57600);
    Serial::enable_transmissitter();

    loop {
        Serial::write_string("hello world\n");
        Serial::write_string("hallo welt\n");

        port_b.set_pin_high(PortB::PIN_13);
        port_b.set_pin_low(PortB::PIN_8);
        helpers::delay(DELAY_DURATION);

        port_b.set_pin_low(PortB::PIN_13);
        port_b.set_pin_high(PortB::PIN_8);
        helpers::delay(DELAY_DURATION);
    }
}
