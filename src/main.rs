#![no_std]
#![no_main]
#![allow(special_module_name)]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::panic::PanicInfo;

mod lib;
mod arduino;

use lib::{ helpers, Port };
use arduino::{ PortB, PortC, PortD, Serial };

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {   
    const DELAY_DURATION: u32 = 1000000;

    let port_b = PortB {};
    let port_d = PortD {};
    let _port_c = PortC {};

    port_b.set_output(PortB::PIN_13);
    port_d.set_output(PortD::PIN_7);

    Serial::set_baud_rate(57600);
    Serial::enable_transmissitter();

    loop {
        Serial::write_string("hello world\n");
        Serial::write_string("hallo welt\n");

        port_b.set_pin_high(PortB::PIN_13);
        port_d.set_pin_low(PortD::PIN_7);
        helpers::delay(DELAY_DURATION);

        port_b.set_pin_low(PortB::PIN_13);
        port_d.set_pin_high(PortD::PIN_7);
        helpers::delay(DELAY_DURATION);
    }
}
