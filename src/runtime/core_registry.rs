use alloc::string::{String, ToString};
use hashbrown::HashMap;
use spin::{Mutex, Once};
use crate::{commands::binaries, runtime::loader::Loader};

pub type CommandFn = fn();

pub struct CoreRegistry {
    // Renamed from 'commands' to 'symbols'
    pub symbols: HashMap<String, CommandFn>,
}

// Renamed to 'REGISTRY' as requested
pub static REGISTRY: Once<Mutex<CoreRegistry>> = Once::new();

/// Internal helper to ensure the registry is initialized safely
pub fn get_registry() -> &'static Mutex<CoreRegistry> {
    REGISTRY.call_once(|| {
        Mutex::new(CoreRegistry {
            symbols: HashMap::new(),
        })
    })
}

/// The actual printing logic that stays ONLY in the kernel.
pub extern "C" fn kernel_sys_print(ptr: *const u8, len: usize) {
    // Safety: We assume the command passed a valid UTF-8 pointer
    let s = unsafe { 
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len)) 
    };
    crate::print!("{}", s); 
}

// In your registry initialization function:
reg.symbols.insert("sys_print".into(), kernel_sys_print as usize);

pub fn register(name: &str, func: CommandFn) {
    let mut reg = get_registry().lock();
    reg.symbols.insert(name.to_string(), func);
}

pub fn list_symbols() {
    let reg = get_registry().lock();
    for name in reg.symbols.keys() {
        println!("{}", name);
    }
}

pub fn init_commands() {
    // 2. Loop through the statically embedded binaries
    for (name, code) in binaries::EMBEDDED_COMMANDS {
        // Use your CodeAllocator + Registry logic here
        Loader::load_and_register(name, code).unwrap();
    }
}

/// This is the "C" bridge that the command calls.
/// It looks up a symbol name and returns the address of the function.
pub extern "C" fn symbol_lookup(name_ptr: *const u8, len: usize) -> usize {
    // 1. Safely convert the raw pointer from the command into a string slice
    let name = unsafe { 
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(name_ptr, len)) 
    };

    // 2. Lock the registry and look for the symbol
    let reg = get_registry().lock();
    
    // 3. If found, return the function pointer as a usize; otherwise return 0
    reg.symbols.get(name)
        .map(|f| *f as usize)
        .unwrap_or(0)
}