use spin::Mutex;

macro_rules! key_press {
    ($x:expr) => {
        Some(KeyEvent::Pressed($x))
    };
}

macro_rules! key_release {
    ($x:expr) => {
        Some(KeyEvent::Released($x))
    };
}



pub static STATE: Mutex<ModifierState> = Mutex::new(ModifierState::new());

// Link keys like shift left and right
struct KeyPair {
    left: bool,
    right: bool,
}

impl KeyPair {
    const fn new() -> Self {
        KeyPair {
            left: false,
            right: false,
        }
    }

    fn is_pressed(&self) -> bool {
        self.left || self.right
    }
}

// All the modifiers
#[derive(Debug, Copy, Clone)]
pub enum Modifier {
    AltLeft(bool),
    AltRight(bool),
    CapsLock, // toogle
    ControlLeft(bool),
    ControlRight(bool),
    NumLock,    // toggle
    ScrollLock, // toggle
    ShiftLeft(bool),
    ShiftRight(bool),
}

// Global modifier key state
pub struct ModifierState {
    alt: KeyPair,
    caps_lock: bool,
    control: KeyPair,
    num_lock: bool,
    scroll_lock: bool,
    shift: KeyPair,
}

impl ModifierState {
    const fn new() -> Self {
        ModifierState {
            alt: KeyPair::new(),
            caps_lock: false,
            control: KeyPair::new(),
            num_lock: false,
            scroll_lock: false,
            shift: KeyPair::new(),
        }
    }

    fn is_uppercase(&self) -> bool {
        self.shift.is_pressed() ^ self.caps_lock
    }

    /// Apply all of our modifiers to character and convert to String
    pub fn apply_to(&self, ascii: u8) -> u8 {
        if self.is_uppercase() {
            map_to_upper(ascii)
        } else {
            ascii
        }
    }

    pub fn update(&mut self, m: Modifier) {
        use self::Modifier::*;

        match m {
            AltLeft(state) => self.alt.left = state,
            AltRight(state) => self.alt.right = state,
            CapsLock => self.caps_lock = !self.caps_lock,
            ControlLeft(state) => self.control.left = state,
            ControlRight(state) => self.control.right = state,
            NumLock => self.num_lock = self.num_lock,
            ScrollLock => self.scroll_lock = self.num_lock,
            ShiftLeft(state) => self.shift.left = state,
            ShiftRight(state) => self.shift.right = state,
        }
    }
}

#[derive(Debug)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

#[derive(Debug)]
pub enum Key {
    Ascii(u8),
    Meta(Modifier),
    LowerAscii(u8),
}


pub fn map_to_upper(lower: u8) -> u8 {
    if lower.is_ascii_lowercase() {
        lower.to_ascii_uppercase()
    } else {
        match lower {
            b'`' => b'~',
            b'1' => b'!',
            b'2' => b'@',
            b'3' => b'#',
            b'4' => b'$',
            b'5' => b'%',
            b'6' => b'^',
            b'7' => b'&',
            b'8' => b'*',
            b'9' => b'(',
            b'0' => b')',
            b'-' => b'_',
            b'=' => b'+',
            b'[' => b'{',
            b']' => b'}',
            b'\\' => b'|',
            b';' => b':',
            b'\'' => b'"',
            b',' => b'<',
            b'.' => b'>',
            b'/' => b'?',
            _ => 0x0,
        }
    }
}