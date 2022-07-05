use super::io::Port;
use crate::log::LogSink;

const SERIAL_PORT_IO: u16 = 0x3F8;

pub struct Serial {
    port: Port<u8>,
}

impl Serial {
    pub const fn new() -> Serial {
        Serial {
            port: Port::new(SERIAL_PORT_IO),
        }
    }
    pub fn write(&self, buffer: &[u8]) {
        for c in buffer {
            self.port.write(*c)
        }
    }
}

pub struct SerialLogger {
    serial: Serial,
}

impl SerialLogger {
    pub const fn new() -> SerialLogger {
        SerialLogger {
            serial: Serial::new(),
        }
    }
}

impl LogSink for SerialLogger {
    fn write(&self, buffer: &[u8]) {
        self.serial.write(buffer);
    }
}
