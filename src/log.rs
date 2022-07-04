use core::fmt::{Arguments, Error, Result, Write};
use spin::Mutex;

enum Level {
    DEBUG,
    INFO,
    WARNING,
    ERROR,
}

struct Log {
    sink: Option<fn(&[u8])>,
}

impl Log {
    const fn new() -> Log {
        Log { sink: None }
    }

    fn set_sink(&mut self, sink: fn(&[u8])) {
        self.sink = Some(sink)
    }
}

impl Write for Log {
    fn write_str(&mut self, s: &str) -> Result {
        if let Some(sink) = self.sink {
            sink(s.as_bytes());
            Ok(())
        } else {
            Err(Error)
        }
    }
}

static LOGGER: Mutex<Log> = Mutex::new(Log::new());

pub fn set_log_output(sink: fn(&[u8])) {
    LOGGER.lock().set_sink(sink)
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::log::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    LOGGER.lock().write_fmt(args).unwrap();
}
