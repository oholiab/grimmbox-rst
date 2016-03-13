#[macro_use]
extern crate log;
extern crate env_logger;

extern crate gag;

use gag::Redirect;
use std::fs::File;
use std::os::unix::io::{FromRawFd, AsRawFd};

const STDOUT: i32 = 1;
// const STDERR: i32 = 2;



fn main() {
    let logfile = File::create("./somelog.log").unwrap();
    env_logger::init().unwrap();
    let redirect = Redirect::stderr(logfile);
    error!("Hello, stdout!");
    drop(redirect);
}
