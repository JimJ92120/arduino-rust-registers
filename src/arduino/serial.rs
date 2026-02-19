// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=159

use crate::lib::Bits;

pub struct Serial;

impl Bits for Serial {}

#[allow(dead_code)]
impl Serial {
    const FREQUENCY: u32 = 16_000_000;

    const UDR0: *mut u8 = 0xC6 as *mut u8;
    const UBRR0H: *mut u8 = 0xC5 as *mut u8;
    const UBRR0L: *mut u8 = 0xC4 as *mut u8;
    const UCSR0A: *mut u8 = 0xC0 as *mut u8;
    const UCSR0B: *mut u8 = 0xC1 as *mut u8;
    const UCSR0C: *mut u8 = 0xC2 as *mut u8;

    // UCSR0A
    const MPCM0: u8 = 0;
    const U2X0: u8 = 1;
    const UPE0: u8 = 2;
    const DOR0: u8 = 3;
    const FE0: u8 = 4;
    const UDRE0: u8 = 5;
    const TXC0: u8 = 6;
    const RXC0: u8 = 7;
    // UCSR0B
    const TXB80: u8 = 0;
    const RXB80: u8 = 1;
    const UCSZ02: u8 = 2;
    const TXEN0: u8 = 3;
    const RXEN0: u8 = 4;
    const UDRIE0: u8 = 5;
    const TXCIE0: u8 = 6;
    const RXCIE0: u8 = 7;
    // UCSR0C
    const UCPOL0: u8 = 0;
    const UCSZ00: u8 = 1; // UCPHA0
    const UCSZ01: u8 = 2; // UDORD0
    const USBS0: u8 = 3;
    const UPM00: u8 = 4;
    const UPM01: u8 = 5;
    const UMSEL00: u8 = 6;
    const UMSEL01: u8 = 7;

    pub fn set_baud_rate(baud_rate: u32) {
        Self::clear_bits(Serial::UBRR0H);
        Self::set_bits_value(Serial::UBRR0L, Serial::calculate_baud_rate(baud_rate));
    }

    pub fn enable_transmissitter() {
        Self::set_bits_shift(Serial::UCSR0B, Serial::TXEN0);
    }

    pub fn write_string(content: &str) {
        for character in content.bytes() {
            Self::write_byte(character);
        }
    }

    pub fn write_byte(byte: u8) {
        while !Self::is_data_register_empty() {}

        Self::set_bits_value(Serial::UDR0, byte);
    }

    fn calculate_baud_rate(baud_rate: u32) -> u8 {
        Serial::FREQUENCY.div_euclid(
            Serial::FREQUENCY.div_euclid(1_000_000) * baud_rate
        ) as u8
    }

    fn is_data_register_empty() -> bool {
        if Self::get_address_value(Self::UCSR0A) & (1 << Self::UDRE0) != 0 {
            true
        } else {
            false
        }
    }
}
