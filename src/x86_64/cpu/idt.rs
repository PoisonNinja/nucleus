use core::{
    arch::asm,
    mem::size_of,
    ops::{Index, IndexMut},
};

use super::gdt;

const IDT_SIZE: usize = 256;

#[repr(C, packed)]
struct Descriptor {
    size: u16,
    offset: u64,
}

impl Descriptor {
    const fn new(size: u16, offset: u64) -> Descriptor {
        Descriptor { size, offset }
    }
    unsafe fn load(&self) {
        asm!(
            "lidt [{descriptor}]",
            descriptor = in(reg) self
        );
    }
}

enum GateType {
    InterruptGate,
    TrapGate,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Entry {
    offset_low: u16,
    selector: u16,
    attributes: u16,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

const fn generate_attributes(dpl: u8, gate_type: GateType, ist: u8) -> u16 {
    let gate_type: u16 = match gate_type {
        GateType::InterruptGate => 0xE,
        GateType::TrapGate => 0xF,
    };
    (1u16 << 15) | ((dpl as u16) << 13) | (gate_type << 8) | ((ist as u16) << 0)
}

const fn split_handler_address(handler: u64) -> (u32, u16, u16) {
    let high = (handler >> 32 & 0xFFFFFFFF) as u32;
    let mid = (handler >> 16 & 0xFFFF) as u16;
    let low = (handler & 0xFFFF) as u16;
    (high, mid, low)
}

impl Entry {
    const fn new() -> Entry {
        Entry {
            offset_low: 0,
            selector: 0,
            attributes: generate_attributes(0, GateType::InterruptGate, 0),
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
    fn set_handler(&mut self, handler: u64, selector: u16) {
        let (high, mid, low) = split_handler_address(handler);
        self.offset_low = low;
        self.offset_mid = mid;
        self.offset_high = high;
        self.selector = selector;
    }
}

struct Table {
    entries: [Entry; IDT_SIZE],
}

impl Table {
    const fn new() -> Table {
        Table {
            entries: [Entry::new(); IDT_SIZE],
        }
    }
    fn load(&self) {
        let descriptor = Descriptor::new(
            (size_of::<Entry>() * IDT_SIZE - 1) as u16,
            &self.entries as *const _ as u64,
        );
        unsafe {
            descriptor.load();
        }
    }
}

impl Index<usize> for Table {
    type Output = Entry;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.entries[index];
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.entries[index];
    }
}

// Must be mut to allow the CPU to write to the accessed bits, safe to access
// since this is only initialized once at boot
static mut IDT: Table = Table::new();

pub fn init() {
    unsafe {
        IDT.load();
    }
}
