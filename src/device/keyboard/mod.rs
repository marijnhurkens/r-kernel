use alloc::collections::VecDeque;
/// This package converts a scancode to a keypress
///
use spin::Mutex;

use device::keyboard::helpers::Key::*;
use device::keyboard::helpers::Modifier::*;
use device::keyboard::helpers::{Key, KeyEvent, STATE};

#[macro_use]
pub mod helpers;

pub struct Keyboard {
    scancode_buffer: VecDeque<u8>,
}

fn get_key(scancode: u64) -> Option<Key> {
    match get_key_event(scancode) {
        Some(KeyEvent::Pressed(key)) => Some(key),
        Some(KeyEvent::Released(key)) => Some(key),
        _ => None,
    }
}

fn get_key_event(scancode: u64) -> Option<KeyEvent> {
    match_scancode(scancode)
}

#[derive(Debug)]
pub struct KeyPackage {
    pub key: Key,
    pub character: Option<char>,
}

impl Keyboard {
    pub fn process_scancode(&mut self) -> Option<KeyPackage> {
        let scancode = match self.scancode_buffer.pop_front() {
            Some(scancode) => scancode,
            None => return None,
        };

        // If multibyte todo: implement
        if scancode == 0xE0 || scancode == 0xE1 {
            let scancode_2 = match self.scancode_buffer.pop_front() {
                Some(scancode) => scancode,
                None => return None,
            };

            return None;
        }

        let key = match get_key(scancode as u64) {
            Some(key) => key,
            None => return None,
        };

        let mut character: Option<char> = None;

        match key {
            Key::Ascii(ascii) => {
                character = Some(ascii as char);
            }
            Key::Meta(modifier) => STATE.lock().update(modifier),
            Key::LowerAscii(lower) => {
                character = Some(STATE.lock().apply_to(lower) as char);
            }
        }

        let key_package = KeyPackage {
            key: key,
            character: character,
        };

        Some(key_package)
    }

    pub fn queue_scancode(&mut self, scancode: u8) {
        self.scancode_buffer.push_back(scancode);
    }
}

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard {
        scancode_buffer: VecDeque::new(),
    });
}

fn match_scancode(scancode: u64) -> Option<KeyEvent> {
    let idx = scancode as usize;

    match scancode {
        0x02...0x0D => key_press!(LowerAscii(b"1234567890-="[idx - 0x02])),
        0x10...0x1B => key_press!(LowerAscii(b"qwertyuiop[]"[idx - 0x10])),
        0x1E...0x28 => key_press!(LowerAscii(b"asdfghjkl;'"[idx - 0x1E])),
        0x2C...0x35 => key_press!(LowerAscii(b"zxcvbnm,./"[idx - 0x2C])),
        0x29 => key_press!(LowerAscii(b'`')),
        0x2B => key_press!(LowerAscii(b'\\')),

        // Non-modifiable ASCII keys
        0x01 => key_press!(Ascii(0x1B)),  // escape
        0x0E => key_press!(Ascii(0x8)),   // backspace
        0x0F => key_press!(Ascii(b'\t')), // tab
        0x1C => key_press!(Ascii(b'\n')), // newline
        0x39 => key_press!(Ascii(b' ')),  // space

        // Meta keys
        0x1D => key_press!(Meta(ControlLeft(true))),
        0xE01D => key_press!(Meta(ControlRight(true))),
        0x2A => key_press!(Meta(ShiftLeft(true))),
        0x36 => key_press!(Meta(ShiftRight(true))),
        0x38 => key_press!(Meta(AltLeft(true))),
        0xE038 => key_press!(Meta(AltRight(false))),
        0x3A => key_press!(Meta(CapsLock)),
        0x45 => key_press!(Meta(NumLock)),
        0x46 => key_press!(Meta(ScrollLock)),

        0xAA => key_release!(Meta(ShiftLeft(false))),
        0xB6 => key_release!(Meta(ShiftRight(false))),
        0x9D => key_release!(Meta(ControlLeft(false))),
        0xE09D => key_release!(Meta(ControlRight(false))),
        0xB8 => key_release!(Meta(AltLeft(false))),
        0xE0B8 => key_release!(Meta(AltRight(false))),

        _ => None,
    }
}
