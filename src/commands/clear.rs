use core::fmt::Write;

use crate::drivers::uart::UART;

pub fn clear_cmd() {
    _ = UART.lock().write_str("\x1B[2J\x1B[1;1H");
}