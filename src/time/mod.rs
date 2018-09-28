/// Time module
/// 
/// Keeps track of the time since the system has booted.
use core::sync::atomic::{Ordering, AtomicUsize};
use alloc::sync::Arc;


const PIC_FREQ: f64 = 18.2065;


pub struct Time {
    ticks: Arc<AtomicUsize>,
}

impl Time {
    pub fn tick(&self) {
       self.ticks.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_seconds(&self) -> f64 {
        self.ticks.load(Ordering::SeqCst) as f64 / PIC_FREQ
    }
}

// IMPORTANT: lazy_static is evaluated when first called,
// this means that this onyl can be called after memory init has
// been done...
lazy_static!
{
    pub static ref TIME: Time = {
        let time = Time {
            ticks: Arc::new(AtomicUsize::new(0)),
        };

        time
    };
}