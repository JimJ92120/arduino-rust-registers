use crate::lib::Address;

pub fn set_output(data_direction_register: *mut u8, gpio_value: u8) {
    Address::shift_left(data_direction_register, gpio_value);
}

pub fn set_input(data_direction_register: *mut u8, gpio_value: u8) {
    Address::unshift_left(data_direction_register, gpio_value);
}

pub fn set_high(port_register: *mut u8, gpio_value: u8) {
    Address::shift_left(port_register, gpio_value);
}

pub fn set_low(port_register: *mut u8, gpio_value: u8) {
    Address::unshift_left(port_register, gpio_value);
}

pub fn read_input(pin_register: *mut u8, gpio_value: u8) -> u8 {
    Address::read(pin_register) & (1 << gpio_value)
}
