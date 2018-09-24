use device::{pic8259};

pub fn init() {
    pic8259::init();
}