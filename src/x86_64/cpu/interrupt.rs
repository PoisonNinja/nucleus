pub struct InterruptContext {
    pub r15: u64,
    pub r14: u64,
    pub r12: u64,
    pub r13: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
    pub err_code: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

macro_rules! pusha {
    () => {
        "
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
        "
    };
}

macro_rules! popa {
    () => {
        "
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
        "
    };
}

macro_rules! interrupt_entry {
    () => {
        concat!("cld\n", pusha!(), "mov rdi, rsp\n")
    };
}

macro_rules! interrupt_exit {
    () => {
        concat!(popa!(), "add rsp, 8\n", "iretq\n")
    };
}

// This macro is based off of Redox's interrupt_handler
#[macro_export]
macro_rules! interrupt_handler {
    ($name:ident, |$context:ident| $code:block) => {
        #[naked]
        extern "C" fn $name() {
            fn inner($context: &$crate::x86_64::cpu::interrupt::InterruptContext) {
                $code
            }
            unsafe {
                ::core::arch::asm!(
                    concat!("push 0\n", interrupt_entry!(), "call {}\n", interrupt_exit!()),
                    sym inner,
                    options(noreturn)
                );
            }
        }
    };
}

#[macro_export]
macro_rules! interrupt_error_handler {
    ($name:ident, |$context:ident| $code:block) => {
        #[naked]
        extern "C" fn $name() {
            fn inner($context: &$crate::x86_64::cpu::interrupt::InterruptContext) {
                $code
            }
            unsafe {
                ::core::arch::asm!(
                    concat!(interrupt_entry!(), "call {}\n", interrupt_exit!()),
                    sym inner,
                    options(noreturn)
                );
            }
        }
    };
}
