/// This is a rust wrapper around the assembly which saves
/// the registers, swaps the process stacks and jumps
use x86_64::registers::rflags::RFlags;

global_asm!(include_str!("context_switch.asm"));

extern "C" {
    fn x86_64_context_switch(prev: *mut Context, next: *const Context);
}

// The context contains the current proccess registers and processor flags
#[derive(Clone, Debug)]
#[repr(C)]
struct Context {
    rflags: usize, // cpu flags register
    rbx: usize, // 64-bit bx register
    r12: usize,
    r13: usize,
    r14: usize,
    rbp: usize, // location base of stack
    rsp: usize, // location of current stack pointer
}

impl Context {
    pub const fn empty() -> Context {
        Context {
            rflags: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            rbp: 0,
            rsp: 0,
        }
    }

    pub fn new(stack_top: *mut u8, proc_entry: usize) -> Context {
        let mut ctx = Context {
            rflags: RFlags::INTERRUPT_FLAG.bits() as usize,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            rbp: stack_top as _,
            rsp: stack_top as usize
        };

        // Push return address and process address


        ctx
    }
}