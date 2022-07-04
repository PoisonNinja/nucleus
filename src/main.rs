#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod log;
mod x86_64;

use core::panic::PanicInfo;

pub fn kmain() -> ! {
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}