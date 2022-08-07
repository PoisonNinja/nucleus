use core::arch::asm;

interrupt_handler!(divide_by_zero, |ctx| {
    error!("Exception: Divide-by-zero at {:x}", ctx.rip);
    panic!()
});

interrupt_handler!(debug, |ctx| {
    let dr6: u64;
    unsafe {
        asm!("mov {}, dr6", lateout(reg) dr6);
    }
    error!("Exception: Debug at {:x} with DR6 = {:x}", ctx.rip, dr6);
    panic!()
});

interrupt_handler!(nmi, |ctx| {
    error!("Exception: NMI at {:x}", ctx.rip);
    panic!()
});

interrupt_handler!(breakpoint, |ctx| {
    error!("Exception: Breakpoint at {:x}", ctx.rip);
    panic!()
});

interrupt_handler!(overflow, |ctx| {
    error!("Exception: Overflow at {:x}", ctx.rip);
    panic!()
});

interrupt_handler!(bound_range_exceeded, |ctx| {
    error!("Exception: Bound range exeeded at {:x}", ctx.rip);
    panic!()
});

interrupt_handler!(invalid_opcode, |ctx| {
    error!("Exception: Invalid opcode at {:x}", ctx.rip);
    panic!()
});

interrupt_handler!(device_not_available, |ctx| {
    error!("Exception: Device not available at {:x}", ctx.rip);
    panic!()
});

interrupt_error_handler!(double_fault, |ctx| {
    error!("Exception: Double fault at {:x}", ctx.rip);
    panic!()
});

interrupt_error_handler!(invalid_tss, |ctx| {
    error!("Exception: Invalid TSS at {:x}", ctx.rip);
    panic!()
});

interrupt_error_handler!(segment_not_present, |ctx| {
    error!("Exception: Segment {:x} not present", ctx.err_code);
    panic!()
});

interrupt_error_handler!(stack_segment_fault, |ctx| {
    error!("Exception: Stack segment {:x} is invalid", ctx.err_code);
    panic!()
});

interrupt_error_handler!(general_protection_fault, |ctx| {
    if ctx.err_code != 0 {
        error!(
            "Exception: General protection fault at {:x} with segment {:x}",
            ctx.rip, ctx.err_code
        );
    } else {
        error!("Exception: General protection fault at {:x}", ctx.rip);
    }
    panic!()
});

interrupt_error_handler!(page_fault, |ctx| {
    let cr2: u64;
    unsafe {
        asm!("mov {}, cr2", lateout(reg) cr2);
    }
    error!(
        "Exception: Page fault triggered by access to {:x} with error code {:x} while running {:x}",
        cr2, ctx.err_code, ctx.rip
    );
    panic!();
});

interrupt_handler!(x87_fpu_exception, |ctx| {
    error!("Exception: x87 FPU exception at {:x}", ctx.rip);
    panic!()
});

interrupt_error_handler!(alignment_check, |ctx| {
    error!("Exception: Alignment check at {:x}", ctx.rip);
    panic!()
});

interrupt_handler!(machine_check, |ctx| {
    error!("Exception: Machine check at {:x}", ctx.rip);
    panic!()
});

interrupt_handler!(simd_fpu_exception, |ctx| {
    error!("Exception: SIMD FPU exception at {:x}", ctx.rip);
    panic!()
});
