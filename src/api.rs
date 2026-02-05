#![no_std]

#[repr(C)]
pub struct CoreRegistry {
    pub symbol_lookup: extern "C" fn(name_ptr: *const u8, len: usize) -> usize,
}

/// The System Call Bridge
pub unsafe fn sys_print(reg: &CoreRegistry, s: &str) {
    let lookup = reg.symbol_lookup;
    // Ask the kernel for the address of "sys_print"
    let addr = lookup("sys_print".as_ptr(), 9);
    
    if addr != 0 {
        let f: extern "C" fn(*const u8, usize) = core::mem::transmute(addr);
        // Hand off the raw pointer and length to the kernel
        f(s.as_ptr(), s.len());
    }
}