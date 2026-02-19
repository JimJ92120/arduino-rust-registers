pub struct Serial;

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
        unsafe {
            core::ptr::write_volatile(Serial::UBRR0H, 0);
            core::ptr::write_volatile(Serial::UBRR0L, Serial::calculate_baud_rate(baud_rate));
        }
    }

    pub fn enable_transmissitter() {
        unsafe {
            core::ptr::write_volatile(Serial::UCSR0B, 1 << Serial::TXEN0);
        }
    }

    pub fn write_character(character: char) {
        unsafe {
            core::ptr::write_volatile(Serial::UDR0, character as u8);
        }
    }

    fn calculate_baud_rate(baud_rate: u32) -> u8 {
        Serial::FREQUENCY.div_euclid(
            Serial::FREQUENCY.div_euclid(1_000_000) * baud_rate
        ) as u8
    }
}
