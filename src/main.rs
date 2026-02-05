// crOS - Core Registry Operating System

#![no_std]
#![no_main]

#[macro_use]
mod macros;

mod arch;
mod commands;
mod drivers;
mod parser;
mod runtime;
mod api;

use crate::{parser::CommandParser, runtime::allocator};
extern crate alloc;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    allocator::init_heap();
    println!("crOS v0.1.0pre");

    runtime::core_registry::init_commands();

    let mut core = CommandParser::new();
    core.write_prompt();
    // The Core Loop
    loop {
        // Poll for characters and execute commands from core.rs
        core.spin();
    }
}