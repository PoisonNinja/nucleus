use super::PAGE_SIZE;
use limine::{LimineMemmapEntry, LimineMemoryMapEntryType, NonNullPtr};

pub struct Allocator<'a> {
    mmap: &'a mut [NonNullPtr<LimineMemmapEntry>],
}

impl<'a> Allocator<'_> {
    pub fn new(mmap: &mut [NonNullPtr<LimineMemmapEntry>]) -> Allocator {
        Allocator { mmap }
    }

    pub fn alloc(&mut self) -> Option<u64> {
        /*
         * Per the Limine specification (https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md#memory-map-feature),
         * the base AND length of usable and bootloader reclaimable regions are
         * guaranteed to be 4096 byte aligned, so we don't need to do any
         * further work to take care of that.
         */
        for mem in self.mmap.iter_mut() {
            if mem.typ == LimineMemoryMapEntryType::Usable && mem.len >= PAGE_SIZE as u64 {
                let ret = mem.base;
                mem.base += PAGE_SIZE as u64;
                mem.len -= PAGE_SIZE as u64;
                return Some(ret);
            }
        }
        None
    }
}
