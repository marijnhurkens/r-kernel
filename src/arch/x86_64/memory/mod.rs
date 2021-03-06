use bootloader::bootinfo::BootInfo;
use x86_64::structures::paging::{
    mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, RecursivePageTable, Size4KiB,
};
use x86_64::VirtAddr;

use self::area_frame_allocator::AreaFrameAllocator;
use self::heap::{HEAP_SIZE, HEAP_START};
use self::stack_allocator::Stack;

mod area_frame_allocator;
pub mod heap;
mod stack_allocator;

/// Initializes the memory controller.
///
/// We want the MemoryController to have the same lifetime as the RecursivePageTable
pub fn init<'a>(
    _boot_info: &BootInfo,
    mut recursive_page_table: RecursivePageTable<'a>,
) -> MemoryController<'a> {
    assert_has_not_been_called!("Memory should only be initialized once!");

    kprintln!("HEAP START = 0x{:X}", HEAP_START);
    kprintln!("HEAP END = 0x{:X}", HEAP_START + HEAP_SIZE);

    let mut frame_allocator = AreaFrameAllocator::new(&_boot_info.memory_map);

    let heap_start_page = Page::containing_address(VirtAddr::new(HEAP_START));
    // Subtract one to get the last frame.
    let heap_end_page = Page::containing_address(VirtAddr::new(HEAP_START + HEAP_SIZE - 1));

    kprintln!("Mapping kernel heap");

    // Map the heap
    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
        map_page(page, flags, &mut recursive_page_table, &mut frame_allocator)
            .expect("Heap page mapping failed");
    }

    kprintln!(
        "HEAP start, page start virt addr: {:?}",
        heap_start_page.start_address()
    );
    kprintln!(
        "HEAP start, page start phys frame: {:?}",
        recursive_page_table.translate_page(heap_start_page)
    );
    kprintln!(
        "HEAP end, page start virt addr: {:?}",
        heap_end_page.start_address()
    );
    kprintln!(
        "HEAP end, page start phys frame: {:?}",
        recursive_page_table.translate_page(heap_end_page)
    );

    // Map the stack
    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100; // 100 pages = 400KB
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start, stack_alloc_end);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    MemoryController {
        page_table: recursive_page_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator,
    }
}

/// Wrapper for the AreaFrameAllocator
pub fn map_page<'a, A>(
    page: Page<Size4KiB>,
    flags: PageTableFlags,
    page_table: &mut RecursivePageTable<'a>,
    frame_allocator: &mut A,
) -> Result<(), MapToError<Size4KiB>>
where
    A: FrameAllocator<Size4KiB>,
{
    let frame = frame_allocator
        .allocate_frame()
        .expect("OOM - Cannot allocate frame");

    unsafe {
        page_table
            .map_to(page, frame, flags, frame_allocator)?
            .flush();
    }

    Ok(())
}

pub struct MemoryController<'a> {
    page_table: RecursivePageTable<'a>,
    frame_allocator: AreaFrameAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

impl<'a> MemoryController<'a> {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController {
            ref mut page_table,
            ref mut frame_allocator,
            ref mut stack_allocator,
        } = self;
        stack_allocator.alloc_stack(page_table, frame_allocator, size_in_pages)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[should_panic]
    // Stack overflow test that could corrupt memory below stack
    // Issue: Use stack probes to check required stack pages before function
    // Tracking: https://github.com/rust-lang/rust/issues/16012
    fn stack_overflow() {
        let _x = [0; 99999];
    }
}
