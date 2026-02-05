#![no_std]
#![feature(alloc_error_handler)] // If you use an allocator

#[macro_use]      // This allows the rest of the lib to see the macros
pub mod macros;

pub mod binaries {
    // This tells the compiler to look in the generated file
    // but won't crash the build if the file is being updated.
    include!("binaries/mod.rs");
}

// Re-export crates so commands can see them
pub extern crate alloc;
pub extern crate spin;
pub extern crate hashbrown;
pub extern crate linked_list_allocator;

// Declare your modules so the compiler finds UART0_BASE, etc.
pub mod arch;
pub mod drivers;
pub mod runtime;
pub mod api;

// Make sure your arch module is visible
pub use arch::pi4b::*;