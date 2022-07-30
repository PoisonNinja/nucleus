use super::gdt;
use super::idt;

pub fn init() {
    gdt::init();
    idt::init();
}
