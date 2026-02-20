// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=159

pub struct USART;

#[allow(dead_code)]
impl USART {
    pub const UDR0: *mut u8 = 0xC6 as *mut u8;
    pub const UBRR0H: *mut u8 = 0xC5 as *mut u8;
    pub const UBRR0L: *mut u8 = 0xC4 as *mut u8;
    pub const UCSR0A: *mut u8 = 0xC0 as *mut u8;
    pub const UCSR0B: *mut u8 = 0xC1 as *mut u8;
    pub const UCSR0C: *mut u8 = 0xC2 as *mut u8;

    // UCSR0A
    pub const MPCM0: u8 = 0;
    pub const U2X0: u8 = 1;
    pub const UPE0: u8 = 2;
    pub const DOR0: u8 = 3;
    pub const FE0: u8 = 4;
    pub const UDRE0: u8 = 5;
    pub const TXC0: u8 = 6;
    pub const RXC0: u8 = 7;
    // UCSR0B
    pub const TXB80: u8 = 0;
    pub const RXB80: u8 = 1;
    pub const UCSZ02: u8 = 2;
    pub const TXEN0: u8 = 3;
    pub const RXEN0: u8 = 4;
    pub const UDRIE0: u8 = 5;
    pub const TXCIE0: u8 = 6;
    pub const RXCIE0: u8 = 7;
    // UCSR0C
    pub const UCPOL0: u8 = 0;
    pub const UCSZ00: u8 = 1; // UCPHA0
    pub const UCSZ01: u8 = 2; // UDORD0
    pub const USBS0: u8 = 3;
    pub const UPM00: u8 = 4;
    pub const UPM01: u8 = 5;
    pub const UMSEL00: u8 = 6;
    pub const UMSEL01: u8 = 7;
}
