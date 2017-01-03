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
extern crate log;
extern crate term;

pub mod error;
use error::{LogResult, ErrorType, bail};

use log::{Log, LogRecord, LogLevel, LogMetadata, SetLoggerError};
use term::stderr;
use term::color::*;

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
pub fn init_with_level(log_level: LogLevel) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log_level.to_log_level_filter());
        Box::new(Logger { level: log_level })
    })
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
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(LogLevel::Trace)
}

/// The logging structure
pub struct Logger {
    level: LogLevel,
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            if let Err(e) = self.log_result(record) {
                println!("Logging failed: {}", e.description);
            }
        }
    }
}

impl Logger {
    fn log_result(&self, record: &LogRecord) -> LogResult<()> {
        // We have to create a new terminal on each log because Send is not fulfilled
        let mut t = stderr().ok_or_else(|| bail(ErrorType::Internal, &"Could not create terminal."))?;
        t.fg(BRIGHT_BLUE)?;
        write!(t, "[{}] ", record.location().module_path())?;
        match record.level() {
            LogLevel::Error => t.fg(BRIGHT_RED)?,
            LogLevel::Warn => t.fg(BRIGHT_YELLOW)?,
            LogLevel::Info => t.fg(BRIGHT_GREEN)?,
            LogLevel::Debug => t.fg(BRIGHT_CYAN)?,
            LogLevel::Trace => t.fg(BRIGHT_WHITE)?,
        };
        write!(t, "[{:<5}] ", record.level())?;
        t.reset()?;
        writeln!(t, "{}", record.args())?;
        Ok(())
    }
}
