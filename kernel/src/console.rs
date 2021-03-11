use core::fmt::Write;
use core::fmt;
use crate::sbicall::console_putchar;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn printk(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! printk {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::printk(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! printkln {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::printk(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::printk(format_args!(concat!("\x1b[34m", "[INFO] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::printk(format_args!(concat!("\x1b[32m", "[WARN] ",$fmt, "\x1b[0m\n") $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::printk(format_args!(concat!("\x1b[31m", "[ERR] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?));
    };
}