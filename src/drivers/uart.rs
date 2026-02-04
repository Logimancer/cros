// src/drivers/uart.rs
// UART Code for the PL011 UART on the pi4b

use core::ptr::{read_volatile, write_volatile};
use core::fmt;
use spin::Mutex;
#[cfg(feature =  "pi4")]
use crate::arch::pi4b::*; // Import the hardware map

// This is the "single source of truth" for your UART hardware
pub static UART: Mutex<Terminal> = Mutex::new(Terminal::new(UART0_BASE));

pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    let mut terminal = UART.lock();
    terminal.write_fmt(args).unwrap();
}

pub struct Terminal {
    base_addr: *mut u32,
}

// Explicitly allow Terminal to be sent across thread (core) boundaries
unsafe impl Send for Terminal {}

impl fmt::Write for Terminal{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl Terminal {
    pub const fn new(addr: *mut u32) -> Self {
        Self { base_addr: addr }
    }

    pub fn write_byte(&self, byte: u8) {
        unsafe {
            // We use the offsets defined in arch/pi4b.rs
            let fr = self.base_addr.byte_add(UART_FR);
            let dr = self.base_addr.byte_add(UART_DR);
            
            while read_volatile(fr) & FR_TXFF != 0 {}
            write_volatile(dr, byte as u32);
        }
    }

    pub fn read_byte(&self) -> u8 {
        unsafe {
            let fr = self.base_addr.byte_add(UART_FR);
            let dr = self.base_addr.byte_add(UART_DR);
            
            while read_volatile(fr) & FR_RXFE != 0 {}
            (read_volatile(dr) & 0xFF) as u8
        }
    }

}