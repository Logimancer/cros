use crate::drivers::uart::UART;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// In drivers/uart.rs:
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut uart = UART.lock();
    uart.write_fmt(args).unwrap();
}
