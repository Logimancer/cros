use crate::drivers::uart::UART;
use crate::runtime::core_registry::REGISTRY;
pub struct CommandParser {
    buffer: [u8; 64],
    index: usize,
}

impl CommandParser {
    pub fn new() -> Self {
        Self {
            buffer: [0u8; 64],
            index: 0,
        }
    }

    pub fn write_prompt(&mut self) {
        print!("+ ");
    }

    pub fn spin(&mut self) {
        let byte = UART.lock().read_byte();
        
        match byte {
            b'\r' | b'\n' => {
                println!("");
                self.execute();
                self.index = 0;
                self.write_prompt(); // Added to show prompt after command
            }
            8 | 127 => {
                if self.index > 0 {
                    self.index -= 1;
                    // Using print! is cleaner than triple locking
                    print!("{}{}{}", 8 as char, ' ', 8 as char);
                }
            }
            _ => {
                if self.index < self.buffer.len() {
                    self.buffer[self.index] = byte;
                    self.index += 1;
                    print!("{}", byte as char); // Cast to char to echo text, not numbers
                }
            }
        }
    }

fn execute(&mut self) {
    if let Ok(input) = core::str::from_utf8(&self.buffer[..self.index]) {
        let cmd_name = input.trim();
        if cmd_name.is_empty() {
            return;
        }
        
        // 1. Find the function pointer and "copy" it out
        let command_to_run = {
            let reg = REGISTRY.lock();
            
            // Look for the entry and return the function pointer
            let func_ptr = match reg.iter().find(|e| e.name == cmd_name) {
                Some(entry) => Some(entry.func),
                None => None,
            };
            
            // Lock drops here automatically when this block ends!
            func_ptr
        };

        // 2. Execute the function without holding ANY locks
        if let Some(func) = command_to_run {
            func(); 
        } else {
            println!("Unknown: {}", cmd_name);
        }
    }
        self.index = 0;
    }
}
