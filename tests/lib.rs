extern crate mowl;

#[macro_use]
extern crate log;
extern crate term;

#[test]
fn success_log() {
    mowl::init().unwrap();
    info!("Info");
    warn!("Warn");
    debug!("Debug");
    trace!("Trace");
    error!("Error");
}
