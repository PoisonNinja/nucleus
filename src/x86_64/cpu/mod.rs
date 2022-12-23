#[macro_use]
mod interrupt;

mod gdt;
mod idt;
mod init;

pub use init::init;
