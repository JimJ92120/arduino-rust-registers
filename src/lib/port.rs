pub trait Port {
    const PORT_ADDRESS: *mut u8;
    const DDR_ADDRESS: *mut u8;
}
