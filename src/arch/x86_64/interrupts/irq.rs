use x86_64::structures::idt::ExceptionStackFrame;
use x86_64::instructions::port::Port;
use device::{keyboard, pic8259};




pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    //print!(".");
    unsafe {
        pic8259::PICS
            .lock()
            .notify_end_of_interrupt(pic8259::TIMER_INTERRUPT_ID)
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    let scancodeport = Port::new(0x60);

    let scancode: u8 = unsafe { scancodeport.read() };

    let mut keyboard_guard = keyboard::KEYBOARD.lock();
    keyboard_guard.queue_scancode(scancode);


    //println!("Keypress {:?}", key_press);

    unsafe {
        pic8259::PICS
            .lock()
            .notify_end_of_interrupt(pic8259::KEYBOARD_INTERRUPT_ID)
    }
}