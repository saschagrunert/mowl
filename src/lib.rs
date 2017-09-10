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
extern crate log;
extern crate term;
extern crate time;

#[macro_use]
extern crate error_chain;

pub mod error;
use error::*;

use log::{Log, LogRecord, LogLevel, LogMetadata};
use term::stderr;
use term::color::*;
use time::now;

/// Initializes the global logger with a specific `max_log_level`.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate mowl;
/// #
/// # fn main() {
/// mowl::init_with_level(log::LogLevel::Warn).unwrap();
///
/// warn!("A warning");
/// info!("A info message");
/// # }
/// ```
pub fn init_with_level(log_level: LogLevel) -> Result<()> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log_level.to_log_level_filter());
        Box::new(Logger { level: log_level, enable_colors: true })
    })?;
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
/// mowl::init_with_level_and_without_colors(log::LogLevel::Warn).unwrap();
///
/// warn!("A warning");
/// info!("A info message");
/// # }
/// ```
pub fn init_with_level_and_without_colors(log_level: LogLevel) -> Result<()> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log_level.to_log_level_filter());
        Box::new(Logger { level: log_level, enable_colors: false })
    })?;
    Ok(())
}

/// Initializes the global logger with `max_log_level` set to `LogLevel::Trace`.
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
pub fn init() -> Result<()> {
    init_with_level(LogLevel::Trace)
}

/// The logging structure
pub struct Logger {
    level: LogLevel,
    enable_colors: bool,
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            if let Err(e) = self.log_result(record) {
                println!("Logging failed: {}", e);
            }
        }
    }
}

impl Logger {
    fn log_result(&self, record: &LogRecord) -> Result<()> {
        // We have to create a new terminal on each log because Send is not fulfilled
        let mut t = stderr().ok_or_else(|| "Could not create terminal.")?;
        if self.enable_colors {
            t.fg(BRIGHT_BLACK)?;
        }
        write!(t, "[{}] ", now().rfc3339())?;
        if self.enable_colors {
            t.fg(BRIGHT_BLUE)?;
        }
        write!(t, "[{}] ", record.location().module_path())?;
        if self.enable_colors {
            match record.level() {
                LogLevel::Error => t.fg(BRIGHT_RED)?,
                LogLevel::Warn => t.fg(BRIGHT_YELLOW)?,
                LogLevel::Info => t.fg(BRIGHT_GREEN)?,
                LogLevel::Debug => t.fg(BRIGHT_CYAN)?,
                LogLevel::Trace => t.fg(BRIGHT_WHITE)?,
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
