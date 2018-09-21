
//! # Test rust kernel
//! 
//! This file makes it possible to use the kernel as a library
//! which is convinient for testing.

#![feature(asm)]

#![no_std] // don't link the Rust standard library

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;
extern crate pic8259_simple;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

// Export the modules
pub mod gdt;
#[macro_use]
pub mod vga_buffer;
pub mod serial;
pub mod interrupts;
pub mod device;
pub mod arch;


pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}