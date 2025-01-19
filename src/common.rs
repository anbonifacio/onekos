use crate::sbi::*;

pub fn putchar(ch: char) -> Result<(), isize> {
    let _ = sbi_call(ch as usize, 0, 0, 0, 0, 0, 0, 1)?;
    Ok(())
}

pub fn memset(buf: *mut u8, c: u8, n: usize) {
    for i in 0..n {
        unsafe {
            *buf.add(i) = c;
        }
    }
}

pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            let _ = putchar(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::common::Writer, $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}