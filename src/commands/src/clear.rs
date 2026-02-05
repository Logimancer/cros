#![no_std]
#![no_main]

// We point to the shared API definition
#[path = "../../api.rs"]
mod api;

#[no_mangle]
pub extern "C" fn clear_cmd(registry: &api::CoreRegistry) {
    let symbol_name = "clear";
    
    // 1. Resolve the "print" function address from the Kernel
    let print_addr = (registry.symbol_lookup)(symbol_name.as_ptr(), symbol_name.len());
    if print_addr != 0 {
        // 2. Cast that address to a function signature
        let print: extern "C" fn(&str) = unsafe { core::mem::transmute(print_addr) };
        
        // 3. Call the kernel's function!
        print("\x1b[2J\x1b[H");
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }