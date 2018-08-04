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
#[macro_use]
extern crate failure;
extern crate log;
extern crate term;
extern crate time;

use failure::Error;
use log::{Level, LevelFilter, Log, Metadata, Record};
use term::{color::*, stderr};
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
/// ```
pub fn init_with_level(log_level: LevelFilter) -> Result<(), Error> {
    log::set_boxed_logger(Box::new(Logger {
        level: log_level,
        enable_colors: true,
    })).map(|()| log::set_max_level(log_level))?;
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
pub fn init_with_level_and_without_colors(log_level: LevelFilter) -> Result<(), Error> {
    log::set_boxed_logger(Box::new(Logger {
        level: log_level,
        enable_colors: false,
    })).map(|()| log::set_max_level(log_level))?;
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
        let mut t = stderr().ok_or_else(|| format_err!("Could not create terminal."))?;
        if self.enable_colors {
            t.fg(BRIGHT_BLACK)?;
        }
        write!(t, "[{}] ", now().rfc3339())?;
        if self.enable_colors {
            t.fg(BRIGHT_BLUE)?;
        }
        write!(t, "[{}] ", record.module_path().unwrap_or("?"));
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
