use crate::result;
use crate::result::Result;
use crate::system_table::SystemTable;
use core::fmt;
use r_efi::protocols::simple_text_output;

pub struct SimpleTextOutput<'a>(&'a mut simple_text_output::Protocol, &'a mut SystemTable);
impl SimpleTextOutput<'_> {
    /// # Errors
    ///
    /// This method may return an error if the output device is not functioning.
    pub fn reset_without_extension(&mut self) -> Result<()> {
        let s = (self.0.reset)(self.0, false.into());
        result::from_status_and_value(s, ())
    }

    /// # Errors
    ///
    /// This method may return an error if the output device is not functioning.
    pub fn output_string(&mut self, buf: &mut [u16]) -> Result<()> {
        let s = (self.0.output_string)(self.0, buf.as_mut_ptr());
        result::from_status_and_value(s, ())
    }
}
impl<'a> From<&'a mut SystemTable> for SimpleTextOutput<'a> {
    fn from(s: &'a mut SystemTable) -> Self {
        let s_ptr = s.get_ptr();

        // SAFETY: `SystemTable` is created only from the argument of `efi_main`, so the mutable
        // reference to `SimpleTextOutput` is created only once.
        let st = unsafe { aligned_ptr::as_mut(s_ptr) };
        let sto = unsafe { aligned_ptr::as_mut(st.con_out) };
        Self(sto, s)
    }
}
impl fmt::Debug for SimpleTextOutput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleTextOutput").finish()
    }
}
impl fmt::Write for SimpleTextOutput<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Writer::new(self, s).write()
    }
}

struct Writer<'a, 'b> {
    console: &'a mut SimpleTextOutput<'b>,
    s: &'a str,
    buf: [u16; Writer::BUF_LEN + 1],
    index: usize,
}
impl<'a, 'b> Writer<'a, 'b> {
    const BUF_LEN: usize = 128;

    fn new(console: &'a mut SimpleTextOutput<'b>, s: &'a str) -> Self {
        Self {
            console,
            s,
            buf: [0; Writer::BUF_LEN + 1],
            index: 0,
        }
    }

    fn write(mut self) -> fmt::Result {
        ucs2::encode_with(self.s, |c| {
            self.push_char(c).map_err(|_| ucs2::Error::BufferOverflow)
        })
        .map_err(|_| fmt::Error)?;

        self.flush().map_err(|_| fmt::Error)
    }

    fn push_char(&mut self, c: u16) -> Result<()> {
        if is_newline(c) {
            self.push_u16(b'\r'.into())?
        }

        self.push_u16(c)
    }

    fn push_u16(&mut self, c: u16) -> Result<()> {
        self.buf[self.index] = c;

        self.index += 1;

        if self.is_buf_full() {
            self.flush()
        } else {
            Ok(())
        }
    }

    fn is_buf_full(&self) -> bool {
        self.index == Self::BUF_LEN
    }

    fn flush(&mut self) -> Result<()> {
        self.terminate_with_null();
        self.print()?;
        self.clear_buf();
        Ok(())
    }

    fn terminate_with_null(&mut self) {
        self.buf[self.index] = 0;
    }

    fn print(&mut self) -> Result<()> {
        self.console.output_string(&mut self.buf)
    }

    fn clear_buf(&mut self) {
        self.index = 0;
    }
}

fn is_newline(c: u16) -> bool {
    c == u16::from(b'\n')
}
