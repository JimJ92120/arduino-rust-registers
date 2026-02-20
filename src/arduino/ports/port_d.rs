// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=73

use crate::lib::{ Port };

pub struct PortD;

impl Port for PortD {
    const PORT_ADDRESS: *mut u8 = (0x20 | 0x0B) as *mut u8;
    const DDR_ADDRESS: *mut u8 = (0x20 | 0x0A) as *mut u8;
}

#[allow(dead_code)]
impl PortD {
    pub const PIN_0: u8 = 0; // PD0 (RX)
    pub const PIN_1: u8 = 1; // PD1 (TX)
    pub const PIN_2: u8 = 2; // PD2
    pub const PIN_3: u8 = 3; // PD3
    pub const PIN_4: u8 = 4; // PD4 (RX_LED)
    pub const PIN_5: u8 = 5; // PD5 (TX_LED)
    pub const PIN_6: u8 = 6; // PD6
    pub const PIN_7: u8 = 7; // PD7
    // PWM: PD3, PD5, PD6
}
