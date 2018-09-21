#[cfg(target_arch="x86_64")]
#[macro_use]
pub mod x86_64;

// If the target architecture is set to x86_64 we fetch the whole
// module and place it in this module. This way the arch dependant 
// stuff can be called via rust_kernel::arch::example directly.
#[cfg(target_arch="x86_64")]
pub use self::x86_64::*;