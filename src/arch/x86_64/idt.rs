extern crate x86_64;

use x86_64::structures::idt::InterruptDescriptorTable;

use super::gdt;
use super::interrupts::{exceptions, irq};
use device::pic8259;

pub fn init() {
    IDT.load();
}

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint
            .set_handler_fn(exceptions::breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(exceptions::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[pic8259::TIMER_INTERRUPT_ID as usize].set_handler_fn(irq::timer_interrupt_handler);
        idt[pic8259::KEYBOARD_INTERRUPT_ID as usize]
            .set_handler_fn(irq::keyboard_interrupt_handler);

        idt
    };
}
