//! Basic error handling mechanisms

use std::io;
use {log, term};

error_chain! {
    foreign_links {
         Io(io::Error) #[doc="An I/O error."];
         Log(log::SetLoggerError) #[doc="A logger error error."];
         Term(term::Error) #[doc="A terminal error."];
    }
}
