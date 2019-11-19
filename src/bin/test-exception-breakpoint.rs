#![feature(abi_x86_interrupt)]
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

// add the library as dependency (same crate name as executable)
extern crate x86_64;
#[macro_use]
extern crate rust_kernel;
#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use rust_kernel::exit_qemu;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

static BREAKPOINT_HANDLER_CALLED: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start(boot_info_address: usize) -> ! {
    rust_kernel::arch::init(boot_info_address);

    IDT.load();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    // Load the breakpoint called static integer and check if equal to 1
    match BREAKPOINT_HANDLER_CALLED.load(Ordering::SeqCst) {
        1 => serial_println!("ok"),
        0 => {
            serial_println!("failed");
            serial_println!("Breakpoint handler was not called.");
        }
        other => {
            serial_println!("failed");
            serial_println!("Breakpoint handler was called {} times", other);
        }
    }

    unsafe {
        exit_qemu();
    }
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");
    serial_println!("{}", info);

    unsafe {
        exit_qemu();
    }

    loop {}
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

// Handle the breakpoint (increment the handler called static integer)
extern "x86-interrupt" fn breakpoint_handler(_: &mut ExceptionStackFrame) {
    BREAKPOINT_HANDLER_CALLED.fetch_add(1, Ordering::SeqCst);
}
