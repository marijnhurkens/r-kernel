pub mod exceptions;
pub mod irq;

#[inline(always)]
pub fn pause() {
    unsafe {
        asm!("pause");
    }
}
