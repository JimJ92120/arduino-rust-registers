#![no_std]
#![no_main]
#![allow(special_module_name)]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

mod panic;
mod lib;
mod arduino;

mod uart;
mod gpio;

use lib::helpers;
use arduino::{ Port, PortB, PortD };

static FREQUENCY: u32 = 16_000_000;
static BAUD_RATE: u32 = 57_600;
static ENABLE_TRANSMISSION: bool = true;
static ENABLE_RECEPTION: bool = true;

static REGISTER_OFFSET: u8 = 0x20;

#[unsafe(no_mangle)]
pub extern "C" fn main() {   
    const DELAY_DURATION: u32 = 500_000;
    const LED_PIN: u8 = PortB::PB4;
    const BUTTON_PIN: u8 = PortD::PD7;

    uart::init(BAUD_RATE, FREQUENCY, ENABLE_TRANSMISSION, ENABLE_RECEPTION);

    gpio::set_output(PortB::DDR_ADDRESS, LED_PIN);
    gpio::set_input(PortD::DDR_ADDRESS, BUTTON_PIN);

    let mut is_led_on: bool = false;

    loop {
        if 0 == gpio::read_input(PortD::PIN_ADDRESS, BUTTON_PIN) {
            continue;
        }

        uart::send("button pressed\n");

        if !is_led_on{
            is_led_on = true;

            gpio::set_high(PortB::PORT_ADDRESS, LED_PIN);
            uart::send("led on\n");
        } else {
            is_led_on = false;

            gpio::set_low(PortB::PORT_ADDRESS, LED_PIN);
            uart::send("led off\n");
        }

        helpers::delay(DELAY_DURATION);
    }
}
