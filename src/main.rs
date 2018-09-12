#![feature(abi_x86_interrupt)]
#![feature(panic_handler)] // required for defining the panic handler
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate cpuio;
extern crate x86_64;

#[macro_use]
extern crate rust_kernel;
#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;
use rust_kernel::interrupts;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_kernel::gdt::init();
    init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(rust_kernel::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        let timer_interrupt_id = usize::from(interrupts::TIMER_INTERRUPT_ID);
        idt[timer_interrupt_id].set_handler_fn(timer_interrupt_handler);

        let keyboard_interrupt_id = usize::from(interrupts::KEYBOARD_INTERRUPT_ID);
        idt[keyboard_interrupt_id].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    print!(".");
    unsafe {
        interrupts::PICS
            .lock()
            .notify_end_of_interrupt(interrupts::TIMER_INTERRUPT_ID)
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    let mut scancode: u8 = 0;

    unsafe {
        scancode = cpuio::UnsafePort::new(0x60).read();
    };

    print!("{}", scancode);

    unsafe {
        interrupts::PICS
            .lock()
            .notify_end_of_interrupt(interrupts::KEYBOARD_INTERRUPT_ID)
    }
}
