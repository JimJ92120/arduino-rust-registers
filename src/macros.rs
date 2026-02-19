#[macro_export]
macro_rules! bits {
    {} => {
        fn set_bits(address: *mut u8, value: u8) {
            unsafe {
                core::ptr::write_volatile(address, Self::get_address_value(address) | (1 << value));
            }
        }

        fn unset_bits(address: *mut u8, value: u8) {
            unsafe {
                core::ptr::write_volatile(address, Self::get_address_value(address) & !(1 << value));
            }
        }

        fn get_address_value(address: *mut u8) -> u8 {
            unsafe {
                core::ptr::read_volatile(address)
            }
        }
    }
}
