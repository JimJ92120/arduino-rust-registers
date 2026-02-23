use core::arch::asm;

pub fn delay(duration: u32) {
    unsafe {
        for _ in 1..duration {
            asm!("nop");
        }
    }
}
