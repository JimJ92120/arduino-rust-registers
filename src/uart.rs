use crate::lib::Address;
use crate::arduino::UART;

pub fn init(baud_rate: u32, frequency: u32, enable_transmission: bool, enable_reception: bool) {
    // set baud rate
    Address::clear(UART::UBRR0H);
    Address::set(
        UART::UBRR0L,
        calculate_baud_rate_from_frequency(frequency, baud_rate)
    );

    // set data frame format to 8 bits + 1 stop bit
    Address::shift_left(UART::UCSR0C, UART::UCSZ00);
    Address::shift_left(UART::UCSR0C, UART::UCSZ01);

    if enable_transmission {
        Address::shift_left(UART::UCSR0B, UART::TXEN0);
    }

    if enable_reception {
        Address::shift_left(UART::UCSR0B, UART::RXEN0);
    }
}

pub fn write(content: &str) {
    for byte in content.as_bytes() {
        // wait empty buffer
        while !(Address::is_shifted_left(UART::UCSR0A, UART::UDRE0)) {}
        
        Address::set(UART::UDR0, *byte);
    }
}

pub fn read() -> u8 {
    // read if receiving data
    if Address::is_shifted_left(UART::UCSR0A, UART::RXC0) {
        return Address::read(UART::UDR0);
    }

    0
}

fn calculate_baud_rate_from_frequency(frequency: u32, baud_rate: u32) -> u8 {
    ((frequency / (frequency / 1_000_000) / baud_rate) - 1) as u8
}
