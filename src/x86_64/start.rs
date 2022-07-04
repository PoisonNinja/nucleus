use super::serial::Serial;
use crate::log;
use limine::LimineMmapRequest;

static MMAP_REQUEST: LimineMmapRequest = LimineMmapRequest::new(0);
static SERIAL: Serial = Serial::new();

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    MMAP_REQUEST.get_response().get().unwrap();

    log::set_log_output(|output| SERIAL.write(output));
    crate::println!("Hello world!");

    crate::kmain();
}
