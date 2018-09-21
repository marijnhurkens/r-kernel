//! # Test rust kernel
//!
//! This file makes it possible to use the kernel as a library
//! which is convinient for testing.

#![feature(asm, alloc, allocator_api, alloc_error_handler, min_const_fn)]
#![no_std] // don't link the Rust standard library

extern crate bootloader_precompiled;
extern crate linked_list_allocator;
extern crate pic8259_simple;
extern crate spin;
extern crate uart_16550;
extern crate volatile;
extern crate x86_64;

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate once;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

// Export the modules
pub mod gdt;
#[macro_use]
pub mod vga_buffer;
pub mod arch;
pub mod device;
pub mod interrupts;
pub mod serial;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

#[alloc_error_handler]
pub fn rust_oom(info: core::alloc::Layout) -> ! {
    panic!("{:?}", info);
}

use arch::memory::heap::HeapAllocator;

#[global_allocator]
static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();
