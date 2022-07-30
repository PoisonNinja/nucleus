use core::{
    arch::asm,
    mem::size_of,
    ops::{Index, IndexMut},
};

const GDT_SIZE: usize = 5;

#[repr(C, packed)]
struct Descriptor {
    size: u16,
    offset: u64,
}

impl Descriptor {
    const fn new(limit: u16, offset: u64) -> Descriptor {
        Descriptor {
            size: limit,
            offset,
        }
    }
    unsafe fn load(&self, data_segment: u8, code_segment: u8) {
        asm!(
            "lgdt [{descriptor}]",
            "mov ds, {data_segment:x}",
            "mov es, {data_segment:x}",
            "mov ss, {data_segment:x}",
            "push {code_segment}",
            "lea {jump_target}, [1f + rip]",
            "push {jump_target}",
            "retfq",
            "1:",
            descriptor = in(reg) self,
            data_segment = in(reg_abcd) data_segment as u16,
            code_segment = in(reg) code_segment as u64,
            jump_target = lateout(reg) _
        );
    }
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

struct Table {
    entries: [Entry; GDT_SIZE],
}

impl Table {
    const fn new() -> Table {
        Table {
            entries: [Entry::new(0, EntryType::Null); GDT_SIZE],
        }
    }
    fn load(&self, data_segment: u8, code_segment: u8) {
        let descriptor = Descriptor::new(
            (size_of::<Entry>() * GDT_SIZE - 1) as u16,
            &self.entries as *const _ as u64,
        );
        unsafe {
            descriptor.load(data_segment, code_segment);
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
static mut GDT: Table = Table::new();

pub fn init() {
    unsafe {
        GDT[1] = Entry::new(
            0,
            EntryType::Code {
                conforming: false,
                readable: true,
            },
        );
        GDT[2] = Entry::new(
            0,
            EntryType::Data {
                grow_down: false,
                writable: true,
            },
        );
        GDT[3] = Entry::new(
            3,
            EntryType::Code {
                conforming: false,
                readable: true,
            },
        );
        GDT[4] = Entry::new(
            3,
            EntryType::Data {
                grow_down: false,
                writable: true,
            },
        );
        GDT.load(0x10, 0x08);
    }
}
