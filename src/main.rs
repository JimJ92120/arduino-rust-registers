#![no_std]
#![no_main]
#![allow(special_module_name)]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

mod panic;
mod lib;
mod arduino;

mod uart;

use lib::helpers;

static FREQUENCY: u32 = 16_000_000;
static BAUD_RATE: u32 = 57_600;
static ENABLE_TRANSMISSION: bool = true;
static ENABLE_RECEPTION: bool = true;

#[unsafe(no_mangle)]
pub extern "C" fn main() {   
    const DELAY_DURATION: u32 = 1_000_000;

    uart::init(BAUD_RATE, FREQUENCY, ENABLE_TRANSMISSION, ENABLE_RECEPTION);

    loop {
        if let Some(byte) = uart::read() {
            uart::send("received: ");
            uart::send(core::str::from_utf8(&[byte]).unwrap());
            uart::send("\n");
        }

        helpers::delay(DELAY_DURATION);

        uart::send("hello world\n");
        helpers::delay(DELAY_DURATION);
    }
}
