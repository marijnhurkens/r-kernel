
use crate::arch::interrupts;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut, Drop};


/// This disables the interrupts when locked.
/// 
/// Data uses unsafecell to allow inner mutability.
pub struct IrqLock<T :?Sized>{
    data: UnsafeCell<T>,
}

pub struct IrqGuard<'a, T :?Sized + 'a> {
    data: &'a mut T,
    was_enabled: bool,
}

unsafe impl<T: ?Sized + Send> Sync for IrqLock<T> {}
unsafe impl<T: ?Sized + Send> Send for IrqLock<T> {}

impl<T> IrqLock<T> {

    pub const fn new(data: T) -> IrqLock<T> {
        IrqLock {
            data: UnsafeCell::new(data),
        }
    }


    pub fn lock(&self) -> IrqGuard<T> {
        let was_enabled = interrupts::interrupts_enabled();

        if was_enabled {
            interrupts::interrupts_disable();
        }

        IrqGuard {
            data: unsafe{ &mut *self.data.get() },
            was_enabled: was_enabled
        }

    }
}

//
// // Needed why?
// impl<'a, T: ?Sized> IrqGuard<'a, T> {
//     /// Drops self
//     pub fn release(self) {}
// }

// Implement the deref operation for the guard
impl<'a, T: ?Sized> Deref for IrqGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.data
    }
}

// Implement mutable dereference
impl<'a, T: ?Sized> DerefMut for IrqGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.data
    }
}

// Implement drop
impl <'a, T: ?Sized> Drop for IrqGuard<'a, T> {
    fn drop(&mut self) {
        if self.was_enabled {
            interrupts::interrupts_enable();
        }
    }
}