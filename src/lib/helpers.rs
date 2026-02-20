use core::arch::asm;

pub fn delay(duration: u32) {
    unsafe {
        for _ in 1..duration {
            asm!("nop");
        }
    }
}

pub fn calculate_baud_rate_from_frequency(frequency: u32, baud_rate: u32) -> u8 {
    ((frequency / (frequency / 1_000_000) / baud_rate) - 1) as u8
}
