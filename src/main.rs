#![no_std]
#![no_main]
#![allow(special_module_name)]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::panic::PanicInfo;

mod lib;
mod arduino;

use lib::{ helpers, Port, Address };
use arduino::{ PortB, PortC, PortD, USART };

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {   
    const DELAY_DURATION: u32 = 1_000_000;
    const FREQUENCY: u32 = 16_000_000;
    const BAUD_RATE: u32 = 57_600;

    // set pin 13 as output
    Address::shift_left(PortB::DDR_ADDRESS, PortB::PIN_13);
    Address::shift_left(PortB::PORT_ADDRESS, PortB::PIN_13);
    // set pin 7 as output
    Address::shift_left(PortD::DDR_ADDRESS, PortD::PIN_7);
    Address::shift_left(PortD::PORT_ADDRESS, PortD::PIN_7);

    // set baud rate
    Address::clear(USART::UBRR0H);
    Address::set(
        USART::UBRR0L,
        helpers::calculate_baud_rate_from_frequency(FREQUENCY, BAUD_RATE)
    );
    // set data frame format to 8 bits + 1 stop bit
    Address::shift_left(USART::UCSR0C, USART::UCSZ00);
    Address::shift_left(USART::UCSR0C, USART::UCSZ01);
    // enable transmission
    Address::shift_left(USART::UCSR0B, USART::TXEN0);
    // enable reception
    Address::shift_left(USART::UCSR0B, USART::RXEN0);

    let content = b"hello world\n";

    loop {
        for byte in content {
            // wait empty buffer
            while !(Address::is_shifted_left(USART::UCSR0A, USART::UDRE0)) {}
            
            // write to buffer
            Address::set(USART::UDR0, *byte);
        }

        Address::shift_left(PortB::PORT_ADDRESS, PortB::PIN_13);
        Address::shift_left(PortD::PORT_ADDRESS, PortD::PIN_7);
        helpers::delay(DELAY_DURATION);

        // wait reception ready
        if Address::is_shifted_left(USART::UCSR0A, USART::RXC0) {
            // read buffer
            let data = Address::read(USART::UDR0);

            // write to buffer
            Address::set(USART::UDR0, data);
            Address::set(USART::UDR0, b'\n');
        }

        Address::unshift_left(PortB::PORT_ADDRESS, PortB::PIN_13);
        Address::unshift_left(PortD::PORT_ADDRESS, PortD::PIN_7);
        helpers::delay(DELAY_DURATION);
    }
}
