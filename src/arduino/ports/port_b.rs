// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=72

use crate::lib::{ Port };

pub struct PortB;

impl Port for PortB {
    const PORT_ADDRESS: *mut u8 = (0x20 | 0x05) as *mut u8;
    const DDR_ADDRESS: *mut u8 = (0x20 | 0x04) as *mut u8;
}

#[allow(dead_code)]
impl PortB {
    pub const PIN_8: u8 = 0; // PB0
    pub const PIN_9: u8 = 1; // PB1
    pub const PIN_10: u8 = 2; // PB2
    pub const PIN_11: u8 = 3; // PB3
    pub const PIN_12: u8 = 4; // PB4
    pub const PIN_13: u8 = 5; // PB5 (LED_BUILTIN)
    // SPI: PB2, PB3, PB4, PB5
    // PWM: PB1, PB2, PB3
}
