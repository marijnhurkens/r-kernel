use bootloader_precompiled::bootinfo::BootInfo;
use x86_64::structures::paging::{PageTable, RecursivePageTable};

pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod memory;
mod device;

/// Initialize for the x86_64 architecture
pub fn init(boot_info_addres: usize) {
    // We get an address to the boot info from the bootloader, let's cast
    // it to a struct.
    let _boot_info: &BootInfo = unsafe { &*(boot_info_addres as *mut BootInfo) };

    // The bootloader which was compiled by the bootimage crate appends a
    // version. This verison must match the crate we use in the kernel.
    if _boot_info.check_version().is_err() {
        panic!("os_bootinfo version passed by bootloader does not match crate version!");
    }

    // Use the p4 page table address found in the boot info and
    // cast it to the page table struct.
    // For more info see: https://github.com/rust-osdev/x86_64/blob/master/src/structures/paging/page_table.rs
    let mut page_table: &mut PageTable =
        unsafe { &mut *(_boot_info.p4_table_addr as *mut PageTable) };

    let rec_page_table =
        RecursivePageTable::new(&mut page_table).expect("recursive page table creation failed");

    let _memory_controller = memory::init(_boot_info, rec_page_table);

    /*  This piece of unsafe code uses the static declared in /lib.rs and initializes
        the heap. 
    */
    unsafe {
        use self::memory::heap::{HEAP_SIZE, HEAP_START};
        ::HEAP_ALLOCATOR.init(HEAP_START as usize, HEAP_SIZE as usize);
    }

    gdt::init();
    idt::init();
    device::init();
}
