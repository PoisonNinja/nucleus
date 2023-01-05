use core::{
    arch::{asm, global_asm},
    mem::size_of,
    ops::{Index, IndexMut},
};

use super::gdt;
use super::interrupt::Frame;

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
    fn set_handler(&mut self, handler: unsafe extern "C" fn() -> ()) {
        let (high, mid, low) = split_handler_address(handler as u64);
        self.offset_low = low;
        self.offset_mid = mid;
        self.offset_high = high;
        self.selector = gdt::CODE_SEGMENT;
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

global_asm!(
    "
common_isr_entry:
    push rax
    push rbx
    push rcx
    push rdx
    push rbp
    push rdi
    push rsi
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    mov rdi, rsp
    call {}

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rsi
    pop rdi
    pop rbp
    pop rdx
    pop rcx
    pop rbx
    pop rax

    add rsp, 16
    iretq
    "
, sym arch_interrupt_handler);

extern "C" fn arch_interrupt_handler(frame: &Frame) {
    info!("Got interrupt {}!", frame.int_no);
}

macro_rules! declare_interrupt_handler {
    ($name:ident, $num:expr) => {
        #[naked]
        unsafe extern "C" fn $name() {
            asm!("push $0",
                 "push {}",
                 "jmp common_isr_entry",
                const $num,
            options(noreturn));
        }
    };
}

macro_rules! declare_interrupt_error_handler {
    ($name:ident, $num:expr) => {
        #[naked]
        unsafe extern "C" fn $name() {
            asm!("push {}",
                 "jmp common_isr_entry",
                const $num,
            options(noreturn));
        }
    };
}

declare_interrupt_handler!(isr0, 0);
declare_interrupt_handler!(isr1, 1);
declare_interrupt_handler!(isr2, 2);
declare_interrupt_handler!(isr3, 3);
declare_interrupt_handler!(isr4, 4);
declare_interrupt_handler!(isr5, 5);
declare_interrupt_handler!(isr6, 6);
declare_interrupt_handler!(isr7, 7);
declare_interrupt_error_handler!(isr8, 8);
declare_interrupt_handler!(isr9, 9);
declare_interrupt_error_handler!(isr10, 10);
declare_interrupt_error_handler!(isr11, 11);
declare_interrupt_error_handler!(isr12, 12);
declare_interrupt_error_handler!(isr13, 13);
declare_interrupt_error_handler!(isr14, 14);
declare_interrupt_handler!(isr15, 15);
declare_interrupt_handler!(isr16, 16);
declare_interrupt_error_handler!(isr17, 17);
declare_interrupt_handler!(isr18, 18);
declare_interrupt_handler!(isr19, 19);
declare_interrupt_handler!(isr20, 20);
declare_interrupt_error_handler!(isr21, 21);
declare_interrupt_handler!(isr22, 22);
declare_interrupt_handler!(isr23, 23);
declare_interrupt_handler!(isr24, 24);
declare_interrupt_handler!(isr25, 25);
declare_interrupt_handler!(isr26, 26);
declare_interrupt_handler!(isr27, 27);
declare_interrupt_handler!(isr28, 28);
declare_interrupt_error_handler!(isr29, 29);
declare_interrupt_error_handler!(isr30, 30);
declare_interrupt_handler!(isr31, 31);

// Must be mut to allow the CPU to write to the accessed bits, safe to access
// since this is only initialized once at boot
static mut IDT: Table = Table::new();

pub fn init() {
    unsafe {
        IDT[0].set_handler(isr0);
        IDT[1].set_handler(isr1);
        IDT[2].set_handler(isr2);
        IDT[3].set_handler(isr3);
        IDT[4].set_handler(isr4);
        IDT[5].set_handler(isr5);
        IDT[6].set_handler(isr6);
        IDT[7].set_handler(isr7);
        IDT[8].set_handler(isr8);
        IDT[9].set_handler(isr9);
        IDT[10].set_handler(isr10);
        IDT[11].set_handler(isr11);
        IDT[12].set_handler(isr12);
        IDT[13].set_handler(isr13);
        IDT[14].set_handler(isr14);
        IDT[15].set_handler(isr15);
        IDT[16].set_handler(isr16);
        IDT[17].set_handler(isr17);
        IDT[18].set_handler(isr18);
        IDT[19].set_handler(isr19);
        IDT[20].set_handler(isr20);
        IDT[21].set_handler(isr21);
        IDT[22].set_handler(isr22);
        IDT[23].set_handler(isr23);
        IDT[24].set_handler(isr24);
        IDT[25].set_handler(isr25);
        IDT[26].set_handler(isr26);
        IDT[27].set_handler(isr27);
        IDT[28].set_handler(isr28);
        IDT[29].set_handler(isr29);
        IDT[30].set_handler(isr30);
        IDT[31].set_handler(isr31);
        IDT.load();
    }
}
