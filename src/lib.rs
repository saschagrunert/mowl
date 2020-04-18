//! # Minimal logger with color support
//!
//! ```
//! # #[macro_use] extern crate log;
//! # extern crate mowl;
//! #
//! # fn main() {
//! mowl::init().unwrap();
//! warn!("Warning");
//! # }
//! ```
#![deny(missing_docs)]
extern crate failure;
extern crate log;
extern crate term;
extern crate time;

use failure::Error;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::prelude::*;
use term::{color::*, StderrTerminal};
use time::now;

/// Initializes the global logger with a specific `max_log_level`.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate mowl;
/// #
/// # fn main() {
/// mowl::init_with_level(log::LevelFilter::Warn).unwrap();
///
/// warn!("A warning");
/// info!("A info message");
/// # }
///```
///
/// # Errors
///
/// An error is returned if a logger has already been set.
pub fn init_with_level(log_level: LevelFilter) -> Result<(), Error> {
    log::set_boxed_logger(Box::new(Logger {
        level: log_level,
        enable_colors: true,
    }))
    .map(|()| log::set_max_level(log_level))?;
    Ok(())
}

/// Initializes the global logger with a specific `max_log_level` and
/// without any coloring.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate mowl;
/// #
/// # fn main() {
/// mowl::init_with_level_and_without_colors(log::LevelFilter::Warn).unwrap();
///
/// warn!("A warning");
/// info!("A info message");
/// # }
/// ```
///
/// # Errors
///
/// An error is returned if a logger has already been set.
pub fn init_with_level_and_without_colors(log_level: LevelFilter) -> Result<(), Error> {
    log::set_boxed_logger(Box::new(Logger {
        level: log_level,
        enable_colors: false,
    }))
    .map(|()| log::set_max_level(log_level))?;
    Ok(())
}

/// Initializes the global logger with `max_log_level` set to
/// `LevelFilter::Trace`.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate mowl;
/// #
/// # fn main() {
/// mowl::init().unwrap();
/// warn!("Warning");
/// # }
/// ```
///
/// # Errors
///
/// An error is returned if logger has already been initialized.
pub fn init() -> Result<(), Error> {
    init_with_level(LevelFilter::Trace)
}

/// The logging structure
pub struct Logger {
    level: LevelFilter,
    enable_colors: bool,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Err(e) = self.log_result(record) {
                println!("Logging failed: {}", e);
            }
        }
    }

    fn flush(&self) {}
}

impl Logger {
    fn log_result(&self, record: &Record) -> Result<(), Error> {
        // We have to create a new terminal on each log because Send is not fulfilled
        let mut t = LogSink::new();
        if self.enable_colors {
            t.fg(BRIGHT_BLACK)?;
        }
        write!(t, "[{}] ", now().rfc3339())?;
        if self.enable_colors {
            t.fg(BRIGHT_BLUE)?;
        }
        write!(t, "[{}] ", record.module_path().unwrap_or("?"))?;
        if self.enable_colors {
            match record.level() {
                Level::Error => t.fg(BRIGHT_RED)?,
                Level::Warn => t.fg(BRIGHT_YELLOW)?,
                Level::Info => t.fg(BRIGHT_GREEN)?,
                Level::Debug => t.fg(BRIGHT_CYAN)?,
                Level::Trace => t.fg(BRIGHT_WHITE)?,
            };
        }
        write!(t, "[{}] ", record.level())?;
        if self.enable_colors {
            t.reset()?;
        }
        writeln!(t, "{}", record.args())?;
        Ok(())
    }

    /// Disable coloring output
    pub fn disable_colors(&mut self) {
        self.enable_colors = false;
    }
}

/// Different output implementations for the logger.
enum LogSink {
    /// Stderr Terminal as default
    Terminal(Box<StderrTerminal>),
    /// Stderr as fallback if a terminal cannot be instantiated
    Fallback(std::io::Stderr),
}

impl LogSink {
    fn new() -> Self {
        if let Some(term) = term::stderr() {
            Self::Terminal(term)
        } else {
            Self::Fallback(std::io::stderr())
        }
    }

    fn fg(&mut self, color: Color) -> Result<(), Error> {
        if let Self::Terminal(t) = self {
            t.fg(color)?;
        }
        Ok(())
    }

    fn reset(&mut self) -> Result<(), Error> {
        if let Self::Terminal(t) = self {
            t.reset()?;
        }
        Ok(())
    }
}

/// Implement Write for `LogSink` by forwarding to the underlying Writers
impl std::io::Write for LogSink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::Terminal(t) => t.write(buf),
            Self::Fallback(e) => e.write(buf),
        }
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        match self {
            Self::Terminal(t) => t.write_vectored(bufs),
            Self::Fallback(e) => e.write_vectored(bufs),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::Terminal(t) => t.flush(),
            Self::Fallback(e) => e.flush(),
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match self {
            Self::Terminal(t) => t.write_all(buf),
            Self::Fallback(e) => e.write_all(buf),
        }
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        match self {
            Self::Terminal(t) => t.write_fmt(args),
            Self::Fallback(e) => e.write_fmt(args),
        }
    }
}
