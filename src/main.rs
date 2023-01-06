#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(naked_functions)] // Allow naked functions
#![feature(asm_const)] // Allow const inputs to asm

#[macro_use]
mod log;
mod x86_64;

use core::panic::PanicInfo;

pub fn kmain() -> ! {
    info!("nucleus v{}", env!("CARGO_PKG_VERSION"));
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
