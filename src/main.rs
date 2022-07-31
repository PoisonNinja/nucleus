#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm_sym)]
#![feature(naked_functions)]

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
fn panic(info: &PanicInfo) -> ! {
    if let Some(s) = info.location() {
        error!("Kernel panic from {}:{}", s.file(), s.line());
    } else {
        error!("Kernel panic");
    }
    loop {}
}
