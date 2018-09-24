extern crate pic8259_simple;

use pic8259_simple::ChainedPics;
use spin;

/// A general rule is to reserve the first 32
/// interrupt vectors for exceptions. PIC 1 will use the 33nd interrupt
/// vector.
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub const TIMER_INTERRUPT_ID: u8 = PIC_1_OFFSET;
pub const KEYBOARD_INTERRUPT_ID: u8 = PIC_1_OFFSET + 1;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init() {
    use x86_64::instructions::interrupts;

    unsafe { PICS.lock().initialize() };
    interrupts::enable();
}
