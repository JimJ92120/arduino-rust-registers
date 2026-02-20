// #[allow(dead_code)]
pub struct Address {}

impl Address {
    pub fn set_value(address: *mut u8, value: u8) {
        unsafe {
            core::ptr::write_volatile(address, value);
        }
    }

    pub fn shift(address: *mut u8, value: u8) {
        unsafe {
            core::ptr::write_volatile(address, Self::get_value(address) | (1 << value));
        }
    }

    pub fn unshift(address: *mut u8, value: u8) {
        unsafe {
            core::ptr::write_volatile(address, Self::get_value(address) & !(1 << value));
        }
    }

    pub fn clear(address: *mut u8) {
        unsafe {
            core::ptr::write_volatile(address, 0);
        }
    }

    pub fn get_value(address: *mut u8) -> u8 {
        unsafe {
            core::ptr::read_volatile(address)
        }
    }
}
