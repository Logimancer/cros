use alloc::vec::Vec;
use spin::Mutex;

/// Type alias for a command function
pub type CommandFn = fn();

/// A registry entry linking a string symbol to a function
pub struct CommandEntry {
    pub name: &'static str,
    pub func: CommandFn,
}

pub static REGISTRY: Mutex<Vec<CommandEntry>> = Mutex::new(Vec::new());

pub fn register(name: &'static str, func: CommandFn) {
    REGISTRY.lock().push(CommandEntry { name, func });
}


