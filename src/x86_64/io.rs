use core::arch::asm;
use core::marker::PhantomData;

pub struct Port<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T> Port<T> {
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port,
            phantom: PhantomData,
        }
    }
}

impl Port<u8> {
    pub fn write(&self, val: u8) {
        unsafe {
            asm!("out dx, al",
             in("al") val,
             in("dx") self.port)
        }
    }

    pub fn read(&self) -> u8 {
        let mut val = 0;
        unsafe {
            asm!("in al, dx",
             out("al") val,
             in("dx") self.port)
        }
        val
    }
}
