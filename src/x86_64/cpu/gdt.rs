use core::{arch::asm, mem::size_of};

#[repr(C, packed)]
struct Descriptor {
    limit: u16,
    offset: u64,
}

enum EntryType {
    Null,
    Code { conforming: bool, readable: bool },
    Data { grow_down: bool, writable: bool },
}

// We want 4 KiB granularity and 64-bit descriptors, shifted by 4 to account
// for the 4 lower bits that act as the middle of the base field
const FLAG_BYTE: u8 = ((1 << 3) | (1 << 1)) << 4;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Entry {
    padding_low: [u8; 5],
    access: u8,
    flags: u8,
    padding_high: u8,
}

const fn generate_access_byte(dpl: u8, executable: bool, dc: bool, rw: bool) -> u8 {
    (1 << 7)
        | (dpl << 5)
        | (1 << 4)
        | ((executable as u8) << 3)
        | ((dc as u8) << 2)
        | ((rw as u8) << 1)
}

impl Entry {
    const fn new(dpl: u8, entry_type: EntryType) -> Entry {
        match entry_type {
            EntryType::Null => Entry {
                padding_low: [0; 5],
                access: 0,
                flags: 0,
                padding_high: 0,
            },
            EntryType::Code {
                conforming,
                readable,
            } => Entry {
                padding_low: [0; 5],
                access: generate_access_byte(dpl, true, conforming, readable),
                flags: FLAG_BYTE,
                padding_high: 0,
            },
            EntryType::Data {
                grow_down,
                writable,
            } => Entry {
                padding_low: [0; 5],
                access: generate_access_byte(dpl, false, grow_down, writable),
                flags: FLAG_BYTE,
                padding_high: 0,
            },
        }
    }
}

const GDT_SIZE: usize = 5;
// Must be mut to allow the CPU to write to the accessed bits, safe to access
// since this is only initialized once at boot
static mut GDT_ENTRIES: [Entry; GDT_SIZE] = [
    Entry::new(0, EntryType::Null),
    Entry::new(
        0,
        EntryType::Code {
            conforming: false,
            readable: true,
        },
    ),
    Entry::new(
        0,
        EntryType::Data {
            grow_down: false,
            writable: true,
        },
    ),
    Entry::new(
        3,
        EntryType::Code {
            conforming: false,
            readable: true,
        },
    ),
    Entry::new(
        3,
        EntryType::Data {
            grow_down: false,
            writable: true,
        },
    ),
];

unsafe fn lgdt(descriptor: &Descriptor) {
    // Far jump technique based on x86_64 crate - load the GDT, and then
    // reload the segment registeres
    asm!(
        "lgdt [{}]",
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax",
        "mov rax, 0x8",
        "push rax",
        "lea rax, [1f + rip]",
        "push rax",
        "retfq",
        "1:",
        in(reg) descriptor,
        out("rax") _
    );
}

pub fn init() {
    let descriptor: Descriptor = Descriptor {
        limit: (size_of::<Entry>() * GDT_SIZE - 1) as u16,
        offset: unsafe { &GDT_ENTRIES as *const _ as u64 },
    };
    unsafe {
        lgdt(&descriptor);
    }
}
