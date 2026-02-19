// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=73

use crate::lib::{ Bits, Port };

pub struct PortC;

impl Bits for PortC {}

impl Port for PortC {
    const PORT_ADDRESS: *mut u8 = (0x20 | 0x08) as *mut u8;
    const DDR_ADDRESS: *mut u8 = (0x20 | 0x07) as *mut u8;
}

#[allow(dead_code)]
impl PortC {
    pub const PIN_14: u8 = 0; // PC0
    pub const PIN_15: u8 = 1; // PC1
    pub const PIN_16: u8 = 2; // PC2
    pub const PIN_17: u8 = 3; // PC3
    pub const PIN_18: u8 = 4; // PC4
    pub const PIN_19: u8 = 5; // PC5
    // reset: PC6
    // analog
    // I2C: PC4, PC5
}
