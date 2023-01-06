use super::cpu;
use super::mm::early;
use super::serial::SerialLogger;
use crate::log;
use limine::LimineMemmapRequest;

static MMAP_REQUEST: LimineMemmapRequest = LimineMemmapRequest::new(0);
static SERIAL_LOGGER: SerialLogger = SerialLogger::new();

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let mmap = MMAP_REQUEST.get_response().get_mut().unwrap();

    log::set_log_output(&SERIAL_LOGGER);
    debug!("Begin x86_64 platform init");

    cpu::init();
    let mut early_allocator = early::Allocator::new(mmap.memmap_mut());

    while let Some(addr) = early_allocator.alloc() {
        info!("Obtained address {:X}", addr);
    }

    crate::kmain();
}
