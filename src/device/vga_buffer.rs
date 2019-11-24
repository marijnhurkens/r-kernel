//! # VGA buffer interface
//!
//!  Interface to write to the VGA buffer.
use core::fmt;
use spin::Mutex;
use volatile::Volatile;

/// Allow unused
/// Add traits:
///     - debug allows for pretty printing
///     - clone: add clone trait so the .clone() method can be used
///     - copy: give copy behaviour instead of move
///     - Eq and PartialEq: add compare behaviour
///
/// Represent as u8
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// Default VGA buffer sizes
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// Define the VGA buffer.
///
/// The Volatile crate allows us to wrap a type which allows for volatile memory access.
/// This enables us to write to the memory without ever using it anywhere else in the program,
/// normally the Rust compiler would optimize thise kind of variables away (the program doesn't
/// use it so why write to the memory?).
///
/// The VGA buffer is a piece of memory-mapped IO, so writing to the VGA buffer directly results
/// in the contents appearing on screen.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// ColorCode contains 2 valid VGA text mode color: a foreground and background color.
/// VGA text mode colors only use 4 bits. The first 4 are used for the foreground color and
/// the last 3 or 4 bits are used for the background color. The backgorund color uses the 4th
/// bit either as a bright but or to set blinking on or off.
///
/// VGA text mode has 8 colors which can be modified by a bright bit. The 4th bit is the bright bit.
/// Example: 0011 (0x3) is Cyan, 1011 (0xb) is Light Cyan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

/// Here a new color code is created.
///
/// Example, we want white text with a blue background:
///
/// White: 1111
/// Blue: 0001
///
/// Background as u8:   0000 0001
///
/// Bitshift left 4:    0001 0000
/// Foreground as u8:   0000 1111
/// Bitwise OR (|):     0001 1111
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// In VGA Text mode a screen character is 8 bits to represent the
/// ASCII code point and the last 8 bit for the color mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// The writer structure saves the current column position in screen
/// and saves the current color code.
///
/// Buffer is a mutable reference with a static lifetime
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// We currently cant initiate statics which contain raw pointers
// (like the buffer) at compile time. This will be possible in the future.
//
// We use a static here so the write will be available everywhere in the program.
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        /// We dont have a heap so we cant use box here.
        /// I dont understand this conversion??
        ///
        /// 1. cast 0xb8000 as a raw mutable pointer of type buffer (unsafe)
        /// 2. dereference with *(...)
        /// https://doc.rust-lang.org/book/first-edition/raw-pointers.html#references-and-raw-pointers
        /// 3. borrow to get a mutable reference
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl Writer {
    /// Here we implement a byte writer.
    ///
    /// If the byte is a newline we write a newline.
    /// We also write a newline if the column position exceeds the buffer width.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                // Start at the bottom of the screen
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;

                // Use the write method which is implemented by the Volatile crate.
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.column_position += 1;
            }
        }
    }

    /// This method moves all the rows except the top row on screen on row up.
    /// This creates a new empty line at the bottom of the screen. The first row on screen
    /// is lost.
    fn new_line(&mut self) {
        // iterate over 2nd row till BUFFER_HEIGHT-1
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                // Write current character to previous row.
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        // Since we copied the rows we have a double row at the bottom of the screen.
        // Clear this last row.
        self.clear_row(BUFFER_HEIGHT - 1); // clear last row
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// Wrap the write byte method. Append a newline and replace unprintable ascii
    /// characters with a square.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), // printable ascii or newline
                _ => self.write_byte(0xfe), // print square for non-printable ascii
            }
        }
    }
}

/// Implement the write method so we can use implement the macros
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::device::vga_buffer::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!( $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!( $fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! kprintln {
    () => (print!("\n"));
    ($fmt:expr) => {
        if $crate::HEAP_ALLOCATOR.get_status() {
            println!(concat!("[ {:>4.4} ] ", $fmt), $crate::time::TIME.get_seconds())
        } else {
            println!(concat!("[ no time ] ", $fmt))
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
         if $crate::HEAP_ALLOCATOR.get_status() {
            println!(concat!("[ {:>4.4} ] ", $fmt), $crate::time::TIME.get_seconds(), $($arg)*)
         } else {
            println!(concat!("[ no time ] ", $fmt), $($arg)*)
         }
    };
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;

    WRITER.lock().write_fmt(args).unwrap();
}

/// Tests
///
/// Use a submodule to separate test code form the rest of the module.
/// Cfg(test) ensures that the code only is compiled when testing.
#[cfg(test)]
mod test {
    // Import al parent module items
    use super::*;

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        let buffer = construct_buffer();
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Blue, Color::Magenta),
            // Since we do have a heap in the testing environment we can use box here.
            buffer: Box::leak(Box::new(buffer)),
        }
    }

    fn construct_buffer() -> Buffer {
        use array_init::array_init;

        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
        }
    }

    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Green, Color::Brown),
        }
    }

    /// Our fisrt test. This test writes 2 bytes and then verifies
    /// if the whole VGA text buffer is correct.
    #[test]
    fn write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 1 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'X');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 1 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'Y');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]
    fn write_formatted() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        writeln!(&mut writer, "a").unwrap();
        writeln!(&mut writer, "b{}", "c").unwrap();

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();

                if i == BUFFER_HEIGHT - 3 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'a');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'b');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'c');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i >= BUFFER_HEIGHT - 2 {
                    assert_eq!(screen_char.ascii_character, b' ');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }
}
