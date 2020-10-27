use x86_64::structures::idt::InterruptStackFrame;


pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    kprintln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    kprintln!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}