// src/arch/pi4b.rs
// Description: This is where hardware specific things go

use core::arch::global_asm;
//asm for boot
global_asm!(
    ".section .text._start",
    ".global _start",
    "_start:",
        // Enable FPU/SIMD access (bits 20-21 of cpacr_el1); Needed for propper debug
        "mov x0, #(3 << 20)",
        "msr cpacr_el1, x0",
        "isb", // Ensure the setting takes effect immediately

        // Check Core ID
        // Read core ID from MPIDR_EL1
        "mrs     x0, mpidr_el1",
        "and     x0, x0, #3",      // Mask for lower 2 bits (core ID)
        "cbz     x0, master_core", // If core 0, jump to master_core

    "park:",
        "wfe",                     // Put secondary cores to sleep
        "b       park",            // Infinite loop if they wake up

    "master_core:",
        // Set up the stack pointer
        "ldr x0, =__stack_top",
        "mov sp, x0",

        // 3. Jump to your Rust or 'main' if core ID is zero
        "bl main",

    // 4. If main ever returns, just loop forever
    "1: wfe",
        "b 1b"
);

// PL011 UART Base on RPi4
pub const PL011_BASE: usize = 0x7E20_1000; 
pub const PI_OFFSET: usize = 0x8000_0000; 
pub const UART0_BASE: *mut u32 = (PL011_BASE + PI_OFFSET) as *mut u32;

// Register Offsets
pub const UART_DR: usize = 0x00;   // Data Register
pub const UART_FR: usize = 0x18;   // Flag Register

// Flag Register Bits
pub const FR_RXFE: u32 = 1 << 4;   // Receive FIFO Empty
pub const FR_TXFF: u32 = 1 << 5;   // Transmit FIFO Full