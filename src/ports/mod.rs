mod port_b;

pub use port_b::PortB;

#[allow(dead_code)]
pub trait Port {
    const PORT_ADDRESS: *mut u8;
    const DDR_ADDRESS: *mut u8;

    // bits!();
    fn set_bits_value(address: *mut u8, value: u8);
    fn set_bits_shift(address: *mut u8, value: u8);
    fn unset_bits_shift(address: *mut u8, value: u8);
    fn clear_bits(address: *mut u8);
    fn get_address_value(address: *mut u8) -> u8;

    fn set_output(&self, pin: u8) {
        Self::set_bits_shift(Self::DDR_ADDRESS, pin);
    }
    fn set_input(pin: u8) {
        Self::unset_bits_shift(Self::DDR_ADDRESS, pin);
    }

    fn set_pin_high(&self, pin: u8) {
        Self::set_bits_shift(Self::PORT_ADDRESS, pin);
    }
    fn set_pin_low(&self, pin: u8) {
        Self::unset_bits_shift(Self::PORT_ADDRESS, pin);
    }
}
