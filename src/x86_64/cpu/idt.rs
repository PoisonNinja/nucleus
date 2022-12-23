use core::{
    arch::asm,
    mem::size_of,
    ops::{Index, IndexMut},
};

use super::{exceptions, gdt};

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
    fn set_handler(&mut self, handler: extern "C" fn(), selector: u16) {
        let handler = handler as u64;
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
        IDT[0].set_handler(exceptions::divide_by_zero, gdt::CODE_SEGMENT);
        IDT[1].set_handler(exceptions::debug, gdt::CODE_SEGMENT);
        IDT[2].set_handler(exceptions::nmi, gdt::CODE_SEGMENT);
        IDT[3].set_handler(exceptions::breakpoint, gdt::CODE_SEGMENT);
        IDT[4].set_handler(exceptions::overflow, gdt::CODE_SEGMENT);
        IDT[5].set_handler(exceptions::bound_range_exceeded, gdt::CODE_SEGMENT);
        IDT[6].set_handler(exceptions::invalid_opcode, gdt::CODE_SEGMENT);
        IDT[7].set_handler(exceptions::device_not_available, gdt::CODE_SEGMENT);
        IDT[8].set_handler(exceptions::double_fault, gdt::CODE_SEGMENT);
        // 9 is unused
        IDT[10].set_handler(exceptions::invalid_tss, gdt::CODE_SEGMENT);
        IDT[11].set_handler(exceptions::segment_not_present, gdt::CODE_SEGMENT);
        IDT[12].set_handler(exceptions::stack_segment_fault, gdt::CODE_SEGMENT);
        IDT[13].set_handler(exceptions::general_protection_fault, gdt::CODE_SEGMENT);
        IDT[14].set_handler(exceptions::page_fault, gdt::CODE_SEGMENT);
        // 15 is reserved
        IDT[16].set_handler(exceptions::x87_fpu_exception, gdt::CODE_SEGMENT);
        IDT[17].set_handler(exceptions::alignment_check, gdt::CODE_SEGMENT);
        IDT[18].set_handler(exceptions::machine_check, gdt::CODE_SEGMENT);
        IDT[19].set_handler(exceptions::general_protection_fault, gdt::CODE_SEGMENT);
        IDT.load();
    }
}
