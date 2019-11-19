pub mod bump_allocator;

use core::alloc::{Alloc, GlobalAlloc, Layout};
use core::ptr::NonNull;
use core::sync::atomic::{Ordering, AtomicBool};
use linked_list_allocator::Heap;
use spin::Mutex;

//pub const HEAP_START: u64 = 0o_000_001_000_000_0000;
pub const HEAP_START: u64 = 0x_0400_0000_0000; // 4.398.046.511.104, 4.39TB
pub const HEAP_SIZE: u64 = 1024 * 1024; // 1kb

pub struct HeapAllocator {
    inner: Mutex<Heap>,
    status: AtomicBool,
}

impl HeapAllocator {
    /// Creates an empty heap. All allocate calls will return `None`.
    pub const fn new() -> Self {
        HeapAllocator {
            inner: Mutex::new(Heap::empty()),
            status: AtomicBool::new(false),
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
        self.status.store(true, Ordering::SeqCst);
    }

    pub fn get_status(&self) -> bool {
        self.status.load(Ordering::SeqCst)
    }
}

/// Wrappers for inner Alloc implementation
unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //kprintln!("Allocating {} bytes", layout.size());

        self.inner
            .lock()
            .alloc(layout)
            .ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //kprintln!("Freeing {} bytes", layout.size());

        self.inner
            .lock()
            .dealloc(NonNull::new_unchecked(ptr), layout);
    }
}
