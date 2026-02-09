use crate::api::CoreRegistry;
use crate::runtime::core_registry;
use core::ptr::addr_of_mut;
use core::mem::transmute;

/// This bridge is what we hand to the command. 
pub static CORE_REGISTRY_BRIDGE: CoreRegistry = CoreRegistry {
    // Ensure this field name matches exactly what is in src/api.rs
    symbol_lookup: core_registry::symbol_lookup,
};

pub type CommandEntry = extern "C" fn(&crate::api::CoreRegistry);

pub struct Loader;

impl Loader {
    /// Loads and executes an embedded binary command by name.
    pub fn run(name: &str) {
        let mut command_code: Option<&[u8]> = None;
        
        for (cmd_name, code) in crate::commands::binaries::EMBEDDED_COMMANDS {
            if *cmd_name == name {
                command_code = Some(code);
                break;
            }
        }

        if let Some(code) = command_code {
            // execute is an unsafe fn, so we wrap the call in unsafe
            unsafe {
                Self::execute(code);
            }
        } else {
            crate::println!("Command not found: {}", name);
        }
    }

    /// Prepares the CPU state and jumps to the command code.
    unsafe fn execute(code: &[u8]) {
        // Even inside an unsafe fn, Rust 2024 requires unsafe blocks for unsafe ops
        let entry_point: extern "C" fn(&CoreRegistry) = unsafe { 
            transmute(code.as_ptr()) 
        };

        crate::println!("--- Executing Command ---");

        // The actual jump/call is also an unsafe operation
        entry_point(&CORE_REGISTRY_BRIDGE);

        crate::println!("--- Command Finished ---");
    }

// In your loader:
pub fn load_and_register(name: &'static str, code: &'static [u8]) -> Result<(), ()> {
    let entry: CommandEntry = unsafe { core::mem::transmute(code.as_ptr()) };
    
    // Ensure your register function in core_registry.rs accepts `CommandEntry`
    crate::runtime::core_registry::register(name, entry);
    
    Ok(())
} 
}   