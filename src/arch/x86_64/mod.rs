use bootloader_precompiled::bootinfo::BootInfo;
use x86_64::structures::paging::{PageTable, RecursivePageTable};

pub mod interrupts;
pub mod memory;

/// Initialize for the x86_64 architecture
pub fn init(boot_info_addres: usize) {
    use self::memory::heap::{HEAP_SIZE, HEAP_START};

    // We get an address to the boot info from the bootloader, let's cast
    // it to a struct.
    let _boot_info: &BootInfo = unsafe { &*(boot_info_addres as *mut BootInfo) };

    // The bootloader which was compiled by the bootimage crate appends a
    // version. This verison must match the crate we use in the kernel.
    if _boot_info.check_version().is_err() {
        panic!("os_bootinfo version passed by bootloader does not match crate version!");
    }

    println!("HEAP START = 0x{:X}", HEAP_START);
    println!("HEAP END = 0x{:X}", HEAP_START + HEAP_SIZE);

    for region in _boot_info.memory_map.iter() {
        println!("{:?}", region)
    }

    let mut page_table: &mut PageTable =
        unsafe { &mut *(_boot_info.p4_table_addr as *mut PageTable) };

    let rec_page_table =
        RecursivePageTable::new(&mut page_table).expect("recursive page table creation failed");
}
