//! # Test rust kernel
//!
//! This file makes it possible to use the kernel as a library
//! which is convenient for testing.
#![no_std] // don't link the Rust standard library
#![feature(abi_x86_interrupt, asm, allocator_api, alloc_error_handler, global_asm)]

extern crate bootloader;
extern crate linked_list_allocator;
extern crate pic8259_simple;
extern crate spin;
extern crate uart_16550;
extern crate volatile;
extern crate x86_64;
extern crate alloc;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate once;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

pub mod time;

#[macro_use]
pub mod device;
pub mod arch;
pub mod sync;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

// Do not include when testing, std has an alloc handler :)
#[cfg(not(test))]
#[alloc_error_handler]
pub fn rust_oom(info: core::alloc::Layout) -> ! {
    panic!(
        "Error allocating {} bytes with alignment {}. Out of memory?",
        info.size(),
        info.align()
    );
}

use linked_list_allocator::LockedHeap;

// Todo: make private
#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
