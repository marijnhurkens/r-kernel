use bootloader::bootinfo::{FrameRange, MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{
    FrameAllocator, FrameDeallocator, PhysFrame, frame::PhysFrameRange, Size4KiB,
};
use x86_64::PhysAddr;

pub struct AreaFrameAllocator {
    pub memory_map: MemoryMap,
}

/// Use the boot info memory map to create a area frame allocator
impl AreaFrameAllocator {
    pub fn new(memory_map: &MemoryMap) -> Self {
        let mut mm = MemoryMap::new();
        for reg in memory_map.iter() {
            mm.add_region(reg.clone());
        }

        AreaFrameAllocator { memory_map: mm }
    }
}

unsafe impl FrameAllocator<Size4KiB> for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        // Find the next region with type usable
        let region = &mut self.memory_map
            .iter_mut()
            .find(|region| region.region_type == MemoryRegionType::Usable && (region.range.end_frame_number - region.range.start_frame_number) > 1);

        // Find the associated frames or error if no region could be found.
        // FrameRange is a c struct with a start and end number.
        // See: https://github.com/rust-osdev/os_bootinfo/blob/master/src/memory_map.rs#L103
        //
        // The region is borrowed.
        
        let frame_range: &mut FrameRange = &mut region
            .as_mut()
            .expect("Could not find usable memory region")
            .range;

        // if frame_range.start_frame_number == frame_range.end_frame_number {
        //     let type = &mut region.as_mut().expect("Could not find usable memory region").region_type;
        //     type = &mut MemoryRegionType::InUse;
        // }
        /* Convert the frame range, which consists of the start and end memory addresses, to the physical frames.
           We now have a struct containing the start and end memory addresses of the frame range.

           1. The addresses are casted to a x86_64::PhysAddr

              Physical addresses should have bits 52 to 64 not set, virtual addresses
              should have bit 48 to 64 set to the value of but 47.

           2. The PhysAddr is then put in the PhysFrame::from_start_address:

              a. Checks if the PhysAddr is a valid page start.
              b. Returns the PhysFrame containing the address.
        
           3. The same is done for the end address.
        */
        let mut phys_range = PhysFrameRange::<Size4KiB> {
            start: PhysFrame::from_start_address(PhysAddr::new(frame_range.start_addr())).unwrap(),
            end: PhysFrame::from_start_address(PhysAddr::new(frame_range.end_addr())).unwrap(),
        };

        /*  Here we check if we still have frames left, and if this is the case we
            return the PhysFrame.

            PhysFrameRange has a next function which returns the next frame if
            one is available. It then increments the .start variable with 1.

            The PhysFrameRange.start variable is of the type PhysFrame. This type
            implements an add function which is calle don the +1. This add function
            returns a new PhysFrame by doing:

            1. Take the add value (1 in example) and multiply by the PhysFrame framesize (4kb in example).
            2. Take the current PhysFrame.start address and add the result of step 1.
            3. Return the PhysFrame belonging to the new address by calling PhysFrame::containing_address(new_addr).

            Then we take the frame_range variable and edit the start frame number. Because
            the frame_range is borrowed of the region it automatically updates the memory map. 
            Next time alloc() is called the start_frame_number will be different and the next frame 
            will be allocated.

            Finally we return the allocated PhysFrame.
        */
        if let Some(frame) = phys_range.next() {
            kprintln!(
                "Allocating fr num: {}, last available: {}, phys frame: {:?}",
                frame_range.start_frame_number, frame_range.end_frame_number, frame
            );

            frame_range.start_frame_number =
                phys_range.start.start_address().as_u64() / frame.size();

            Some(frame)
        } else {
            None
        }
    }
}

impl FrameDeallocator<Size4KiB> for AreaFrameAllocator {
    #[allow(unused)]
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        unimplemented!()
    }
}

