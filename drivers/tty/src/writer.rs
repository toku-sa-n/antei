use {
    super::{font, vram},
    bit_field::BitField,
    conquer_once::spin::Lazy,
    core::fmt::{self, Write},
    font8x8::{unicode::BasicFonts, UnicodeFonts},
    rgb::RGB8,
    spinning_top::{const_spinlock, Spinlock},
    vek::Vec2,
};

static BASIC_FONTS: Lazy<BasicFonts> = Lazy::new(BasicFonts::new);

static LOG_WRITER: Spinlock<Writer> = const_spinlock(Writer::new(RGB8::new(0xff, 0xff, 0xff)));

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println{
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*)=>{
        $crate::print!("{}\n",core::format_args!($($arg)*));
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments<'_>) {
    write!(*LOG_WRITER.lock(), "{}", args).unwrap();
}

pub(crate) struct Writer {
    coord: Vec2<u32>,
    color: RGB8,
}
impl Writer {
    pub(crate) const fn new(color: RGB8) -> Self {
        Self {
            coord: Vec2 { x: 0, y: 0 },
            color,
        }
    }

    fn print_str(&mut self, str: &str) {
        for c in str.chars() {
            self.print_char(c);
        }
    }

    fn print_char(&mut self, c: char) {
        if c == '\n' {
            self.break_line();
            return;
        }

        let font = BASIC_FONTS.get(c).expect("Unrecognized character.");

        self.write_char_on_screen(font);
        self.move_cursor_by_one_character();

        if self.cursor_is_outside_screen() {
            self.break_line();
        }
    }

    fn break_line(&mut self) {
        self.carriage_return();
        self.newline();
    }

    fn carriage_return(&mut self) {
        self.coord.x = 0;
    }

    fn newline(&mut self) {
        if self.cursor_is_at_the_bottom() {
            vram::scroll_up();
        } else {
            self.move_cursor_to_next_line();
        }
    }

    fn cursor_is_at_the_bottom(&self) -> bool {
        self.current_line() == Self::num_lines() - 1
    }

    fn current_line(&self) -> u32 {
        self.coord.y / font::HEIGHT
    }

    fn num_lines() -> u32 {
        vram::resolution().y / font::HEIGHT
    }

    fn move_cursor_to_next_line(&mut self) {
        self.coord.y += font::HEIGHT;
    }

    fn move_cursor_by_one_character(&mut self) {
        self.coord.x += font::WIDTH;
    }

    fn cursor_is_outside_screen(&self) -> bool {
        self.coord.x + font::WIDTH >= vram::resolution().x
    }

    fn write_char_on_screen(&self, font: [u8; 8]) {
        for (y, row) in font.iter().enumerate() {
            for x in 0..8 {
                if row.get_bit(x) {
                    let c = self.coord + Vec2::new(x, y).as_();

                    vram::set_color(c.as_(), self.color);
                }
            }
        }
    }
}
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.print_str(s);
        Ok(())
    }
}
