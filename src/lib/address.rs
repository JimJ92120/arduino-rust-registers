pub struct Address {}

#[allow(dead_code)]
impl Address {
    pub fn set(address: *mut u8, value: u8) {
        unsafe {
            core::ptr::write_volatile(address, value);
        }
    }

    pub fn shift_left(address: *mut u8, value: u8) {
        unsafe {
            core::ptr::write_volatile(address, Self::read(address) | (1 << value));
        }
    }

    pub fn unshift_left(address: *mut u8, value: u8) {
        unsafe {
            core::ptr::write_volatile(address, Self::read(address) & !(1 << value));
        }
    }

    pub fn is_shifted_left(address: *mut u8, value: u8) -> bool {
        0 != Self::read(address) & (1 << value)
    }

    pub fn clear(address: *mut u8) {
        unsafe {
            core::ptr::write_volatile(address, 0);
        }
    }

    pub fn read(address: *mut u8) -> u8 {
        unsafe {
            core::ptr::read_volatile(address)
        }
    }
}
