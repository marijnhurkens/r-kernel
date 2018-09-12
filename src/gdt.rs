use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// TSS = TaskStateSegment
/// IST = InterruptStackTable contains 7 stacks
/// Create static TSS, create the IST and include 
/// the double fault stack in the 0th enty of the IST
lazy_static! {
    static ref TSS:TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;

            // No memory management yet so use a static array.
            // Use a static mutable array so the stack remains
            // readable.
            // 
            // TODO: make safe later
            static mut STACK: [u8; STACK_SIZE] = [0;STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;

            // Write the top address of the double fault stack to the IST 0th index
            // We use the top address because x86 stacks grow downwards (high to low)
            stack_end
        };

        tss
    };
}