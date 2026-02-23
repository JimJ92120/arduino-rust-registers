// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=73
use crate::REGISTER_OFFSET;
use super::Port;

pub struct PortD;

impl Port for PortD {
    const PORT_ADDRESS: *mut u8 = (REGISTER_OFFSET | 0x0B) as *mut u8;
    const DDR_ADDRESS: *mut u8 = (REGISTER_OFFSET | 0x0A) as *mut u8;
    const PIN_ADDRESS: *mut u8 = (REGISTER_OFFSET | 0x09) as *mut u8;
}

#[allow(dead_code)]
impl PortD {
    pub const PD0: u8 = 0; // GPIO 0 + RX
    pub const PD1: u8 = 1; // GPIO 1 + TX
    pub const PD2: u8 = 2; // GPIO 2
    pub const PD3: u8 = 3; // GPIO 3
    pub const PD4: u8 = 4; // GPIO 4 + RX_LED
    pub const PD5: u8 = 5; // GPIO 5 + TX_LED
    pub const PD6: u8 = 6; // GPIO 6
    pub const PD7: u8 = 7; // GPIO 7
}
