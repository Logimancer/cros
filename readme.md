# Core Registry Operating System (crOS)

## OS Summary
**Core Registry OS (crOS)** is a minimalist, hardware-direct operating system designed for [Single Address Space (SASOS)](https://en.wikipedia.org/wiki/Single_address_space_operating_system) environments.
Its main objectives are:

- **Flat Memory Sovereignty:** To maintain a completely linear and contiguous address space where [primary memory is the only namespace](https://wiki.c2.com/?SingleAddressSpaceOperatingSystem). By avoiding folders or file systems, every command and data block is just a direct pointer to a specific memory address.
- **Symbolic Direct Action:** To replace the traditional "shell and file execution" model with a **Core Registry**. This central symbol table maps human-readable names (like `about`) directly to their starting [memory addresses](https://wiki.osdev.org/Symbol_Table), allowing the CPU to jump to code without the overhead of loading files from storage.
- **Low-Level Transparency:** To provide the programmer with "concrete" access to the machine. By stripping away [virtual memory abstractions](https://www.infosecinstitute.com/resources/reverse-engineering/memory-models/) and page tables, the OS ensures that the software state is always a one-to-one reflection of the hardware state.
- **Mechanical Interface Fidelity:** To function as a **paper terminal** OS. This requires strict adherence to [Carriage Return (CR)](https://en.wikipedia.org/wiki/Carriage_return) and Line Feed (LF) protocols, ensuring that the system's output is optimized for teletype-style devices where the physical movement of the print head is a primary design constraint.
- **Rust-Powered Safety:** To leverage Rust’s `no_std` and [zero-cost abstractions](https://rust-osdev.com/this-month/2024-08/) to manage these low-level pointers safely, ensuring the "Registry" can be updated or queried without the memory-safety risks typically found in raw assembly or C kernels.

### Documentation
[UART](
https://github.com/raspberrypi/documentation/blob/master/documentation/asciidoc/computers/configuration/uart.adoc)

#### To Debug
```
build+run_gdb.ps1
util\gdb-multiarch\gdb-multiarch.exe .\target\aarch64-unknown-none\debug\chryos
```
#### To get rid of Rust Analyzer's false positive on the panic handler:
create "./vscode/settings.json" with contents:
```
{
    "rust-analyzer.checkOnSave.allTargets": false,
    "rust-analyzer.cargo.target": "aarch64-unknown-none"
}
```

#### Notes
Research ARMv8-A hardware-based protection and virtualization for SASOS.

## rust-objcopy
Check to make sure .o is being processed in binaries

To tell if rust-objcopy is working, you can perform a quick manual check in your terminal. Since rust-objcopy is a proxy for the LLVM tools provided by Rust, it needs two things to be installed correctly.

1. The "Quick Version" Test
Open your terminal and run:

Bash

rust-objcopy --version
If it works: You will see a version string like LLVM (http://llvm.org/): LLVM version 18.x.x.

If it says "command not found": You likely haven't installed the cargo-binutils package.

If it says "Failed to execute tool": You have the package, but you're missing the actual LLVM tools component.

2. How to fix it if it's missing
If the command failed, run these two commands to set it up:

Install the proxy tools:

Bash

cargo install cargo-binutils
Install the LLVM engines:

Bash

rustup component add llvm-tools-preview

#### (Warning)
This is the most "elegant" way, but it introduces one big challenge: The Linker.

When your command calls UART::new(), the command binary expects that function to exist. Since we are only emitting an object file (--emit=obj) and then stripping it to a .bin, you might get "Undefined Symbol" errors.

Why? Because rustc compiles the command and says "I know UART::new exists in the kernel," but it doesn't actually copy the machine code for UART::new into the clear.bin file unless you perform a full link.

How to solve the "Undefined Symbol" error:
If you want the command to actually contain the kernel code it uses, you must change your build.rs to produce a full executable before objcopy:

Change --emit=obj to --crate-type=bin.

Pass a linker script to rustc so it knows where the command's memory starts (usually a different address than the kernel).