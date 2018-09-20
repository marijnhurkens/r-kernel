/// This package converts a scancode to a keypress
///
use spin::Mutex;

pub struct Keyboard {
    make_break: MakeBreak,
    multi_byte: bool,
    // listeners: Vec<Fn<KeyPress>>, // need std lib
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MakeBreak {
    None,
    Make,
    Break,
}

#[derive(Debug)]
pub struct KeyPress {
    pub make_break: MakeBreak,
    pub is_character: bool,
    pub ascii_character: Option<char>,
}

impl Keyboard {
    pub fn process_scancode(&mut self, scancode: u8) -> Result<(), &str> {

        // If multibyte set to multibyte mode and continue
        if scancode == 0xE0 || scancode == 0xE1 {
            self.multi_byte = true;
            return Ok(());
        }

        // At this point we are not dealing with the special 0xE1 and 0xE1 codes
        // so we can process the byte further. First we process the normal 1 byte codes.
        if !self.multi_byte {
            // If higher bit is set its a break scancode
            if (scancode & 128) == 128 {
                self.make_break = MakeBreak::Break;
            } else {
                self.make_break = MakeBreak::Make;
            }

            let is_character = true;
            let ascii_character = 'a';

            // Let's finish up
            let _key_press = KeyPress {
                make_break: self.make_break,
                is_character: is_character,
                ascii_character: Some(ascii_character),
            };
            self.reset(); //reset state

            return Ok(());
        }

        // We have read the multibyte char and are dealing with the next character in the multibyte sequence
        self.reset();

        Ok(())
    }

    pub fn reset(&mut self) {
        self.make_break = MakeBreak::None;
        self.multi_byte = false;
    }
}

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard {
        make_break: MakeBreak::None,
        multi_byte: false,
    });
}
