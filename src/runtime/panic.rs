use core::panic::PanicInfo;
use alloc::string::ToString;

use crate::drivers::uart::UART;
use core::fmt::Write;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    _= UART.lock().write_str(info.message().to_string().as_str());

    loop {
        // AArch64 'Wait For Event' - puts CPU in low power mode until reset
        unsafe { core::arch::asm!("wfe") };
    }
}
