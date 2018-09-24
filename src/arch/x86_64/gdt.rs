use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
//const STACK_SIZE: usize = 4096;

pub fn init() {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();

    unsafe {
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

/// GDT = GlobalDescriptorTable
/// TSS = TaskStateSegment  
/// IST = InterruptStackTable contains 7 stacks
///
/// Create static TSS, create the IST and include
/// the double fault stack in the 0th enty of the IST
lazy_static! {
    //static ref GDT_STACK: Vec<[u8;STACK_SIZE]> = vec!([0;STACK_SIZE]);

    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {


            // No memory management yet so use a static array.
            // Use a static mutable array so the stack remains
            // readable.
            //
            // TODO: make safe later
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;

            // Write the top address of the double fault stack to the IST 0th index
            // We use the top address because x86 stacks grow downwards (high to low)
            stack_end
        };

        tss
    };
}

/// Create the global descriptor table.
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}
