use core::fmt::{Arguments, Error, Result, Write};
use spin::Mutex;

const RESET_STR: &str = "\x1b[39m";

pub trait LogSink: Send + Sync {
    fn write(&self, buffer: &[u8]);
}

pub enum Level {
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
    ($level:path, $($arg:tt)*) => ($crate::log::_print($level, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug {
    () => ($crate::print!($crate::log::Level::DEBUG, "\n"));
    ($($arg:tt)*) => ($crate::print!($crate::log::Level::DEBUG, "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    () => ($crate::print!($crate::log::Level::INFO, "\n"));
    ($($arg:tt)*) => ($crate::print!($crate::log::Level::INFO, "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warning {
    () => ($crate::print!($crate::log::Level::WARNING, "\n"));
    ($($arg:tt)*) => ($crate::print!($crate::log::Level::WARNING, "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    () => ($crate::print!($crate::log::Level::ERROR, "\n"));
    ($($arg:tt)*) => ($crate::print!($crate::log::Level::ERROR, "{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(level: Level, args: Arguments) {
    let (color_code, prefix) = match level {
        Level::DEBUG => ("\x1b[36m", "DEBUG"),
        Level::INFO => ("\x1b[32m", "INFO"),
        Level::WARNING => ("\x1b[33m", "WARNING"),
        Level::ERROR => ("\x1b[31m", "ERROR"),
    };
    LOGGER
        .lock()
        .write_fmt(format_args!(
            "{}[{}]{} {}",
            color_code, prefix, RESET_STR, args
        ))
        .unwrap();
}
