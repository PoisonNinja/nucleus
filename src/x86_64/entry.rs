use limine::LimineMmapRequest;

static MMAP_REQUEST: LimineMmapRequest = LimineMmapRequest::new(0);

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    MMAP_REQUEST.get_response().get().unwrap();

    crate::kmain();
}
