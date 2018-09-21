pub mod bump_allocator;

use core::alloc::{Alloc, GlobalAlloc, Layout};
use core::ptr::NonNull;
use linked_list_allocator::Heap;
use spin::Mutex;

//pub const HEAP_START: u64 = 0o_000_001_000_000_0000;
pub const HEAP_START: u64 = 0o_010_000_000_000_0000;
pub const HEAP_SIZE: u64 = 1024 * 1024 * 10; // 10 MB 

pub struct HeapAllocator {
    inner: Mutex<Heap>,
}

impl HeapAllocator {
    /// Creates an empty heap. All allocate calls will return `None`.
    pub const fn new() -> Self {
        HeapAllocator {
            inner: Mutex::new(Heap::empty()),
        }
    }

    /// Initializes an empty heap
    ///
    /// # Unsafety
    ///
    /// This function must be called at most once and must only be used on an
    /// empty heap.
    pub unsafe fn init(&self, heap_bottom: usize, heap_size: usize) {
        self.inner.lock().init(heap_bottom, heap_size);
    }
}

/// Wrappers for inner Alloc implementation
unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //println!("Allocating {} bytes", layout.size());

        self.inner
            .lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //println!("Freeing {} bytes", layout.size());

        self.inner
            .lock()
            .dealloc(NonNull::new_unchecked(ptr), layout);
    }
}
