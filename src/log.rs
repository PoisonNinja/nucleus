use core::fmt::{Arguments, Error, Result, Write};
use spin::Mutex;

pub trait LogSink: Send + Sync {
    fn write(&self, buffer: &[u8]);
}

enum Level {
    DEBUG,
    INFO,
    WARNING,
    ERROR,
}

struct Log {
    sink: Option<&'static dyn LogSink>,
}

impl Log {
    const fn new() -> Log {
        Log { sink: None }
    }

    fn set_sink(&mut self, sink: &'static dyn LogSink) {
        self.sink = Some(sink)
    }
}

impl Write for Log {
    fn write_str(&mut self, s: &str) -> Result {
        if let Some(sink) = self.sink {
            sink.write(s.as_bytes());
            Ok(())
        } else {
            Err(Error)
        }
    }
}

static LOGGER: Mutex<Log> = Mutex::new(Log::new());

pub fn set_log_output(sink: &'static dyn LogSink) {
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
