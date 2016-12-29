extern crate mowl;
use mowl::error::*;

#[macro_use]
extern crate log;
extern crate term;

use std::io;
use std::error::Error;

#[test]
fn success_convert_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "Not found");
    let log_error: LogError = io_error.into();
    assert_eq!(log_error.code, ErrorType::Other);
    assert_eq!(log_error.description, "Not found".to_string());
}

#[test]
fn success_convert_from_term_error() {
    let term_error = term::Error::NotSupported;
    let log_error: LogError = term_error.into();
    assert_eq!(log_error.code, ErrorType::Other);
    assert_eq!(log_error.description,
               "operation not supported by the terminal".to_string());
    println!("{}", log_error);
    println!("{:?}", log_error);
    println!("{}", log_error.description());
}

#[test]
fn success_log() {
    mowl::init().unwrap();
    info!("Info");
    warn!("Warn");
    debug!("Debug");
    trace!("Trace");
    error!("Error");
}
