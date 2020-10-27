#![feature(abi_x86_interrupt, alloc_error_handler, llvm_asm)]
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate alloc;
extern crate bootloader;
extern crate x86_64;
#[macro_use]
extern crate rust_kernel;

use alloc::string::String;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use rust_kernel::arch;
use rust_kernel::arch::interrupts;
use rust_kernel::device::keyboard::KEYBOARD;


/// The kernel is compiled using the bootimage and bootloader crates.
/// The bootloader crate contains the rather difficult setup for x86_64,
/// this includes setting up long mode (64 bit), enabling paging and various
/// other things. The bootimage crate compiles both the bootloader and the
/// kernel and appends the kernel to the bootloader. The bootloader ensures
/// that the _start function is called when it's finished.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start(boot_info_address: usize) -> ! {
    kprintln!("Rust test kernel starting{}", "...");
    kprintln!("Memory status {}", rust_kernel::HEAP_ALLOCATOR.lock().size());

    // Let's init the kernel
    arch::init(boot_info_address);

    kprintln!("Memory status {}", rust_kernel::HEAP_ALLOCATOR.lock().size());

    let t = String::from("test");

    kprintln!("Test string contents: {}", t);
    kprintln!("Test string ptr: {:?}", t.as_ptr());

    const VECSIZE: usize = 1024 * 2; //2kb
    let mut a: Vec<u8> = Vec::with_capacity(VECSIZE);

    kprintln!("Allocated vec of size {}", VECSIZE);

    for i in 0..VECSIZE {
        a.push(i as u8);
    }

    //kprintln!("Pushed ints to vec.");
    kprintln!("Done... to main loop.");

    //kprintln!("{}", rust_kernel::time::TIME.lock().ticks);

    // Todo:
    // - time system
    // - context structs and functions for cpu contetx switching
    // - tasks: processes, create, pid, switch context
    // - task scheduler
    // - process communication
    // - syscalls: handling processes
    // - create console as process
    // - filesystem?

    // start console program


    loop {
        if let Some(key) = KEYBOARD.lock().process_scancode() {
            if let Some(character) = key.character {
                print!("{}", character);
            } else {
                kprintln!("{:?}", key);
            }
        }
        
        // kprintln!(
        //     "Time: {}", (rust_kernel::time::TIME.get_seconds())
        // );


       interrupts::pause();
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    kprintln!("{}", info);
    loop {}
}


async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}