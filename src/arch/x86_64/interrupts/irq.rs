use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::Port;
use device::{keyboard, pic8259};
use time;

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {

    // This function requires memory to be intialized. But the PIC interrupts are
    // off until the end of arch init() so by now we should have a heap.
    time::TIME.tick();

    unsafe {
        pic8259::PICS
            .lock()
            .notify_end_of_interrupt(pic8259::TIMER_INTERRUPT_ID)
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    let scancodeport = &mut Port::new(0x60);

    let scancode: u8 = unsafe { scancodeport.read() };

    let mut keyboard_guard = keyboard::KEYBOARD.lock();
    keyboard_guard.queue_scancode(scancode);

    unsafe {
        pic8259::PICS
            .lock()
            .notify_end_of_interrupt(pic8259::KEYBOARD_INTERRUPT_ID)
    }
}
