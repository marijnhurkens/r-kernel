use x86_64::registers::rflags;

pub mod exceptions;
pub mod irq;

#[inline(always)]
pub fn pause() {
    unsafe {
        llvm_asm!("pause");
    }
}

#[inline(always)]
pub fn interrupts_disable() {
    unsafe {
        llvm_asm!("cli" : : : : "intel", "volatile");
    }
}

pub fn interrupts_enabled() -> bool {
    rflags::read().contains(rflags::RFlags::INTERRUPT_FLAG)
}

#[inline(always)]
pub fn interrupts_enable() {
    unsafe {
        // Enable interrupts and perform noop because interrupts
        // are only handled after next instruction.
        llvm_asm!("sti; nop" : : : : "intel", "volatile");
    }
}