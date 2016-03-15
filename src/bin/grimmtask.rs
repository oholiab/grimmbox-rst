#[macro_use]
extern crate log;
extern crate env_logger;

extern crate gag;
extern crate rustbox;
extern crate grimmbox;

use gag::Redirect;
use std::fs::File;
use std::os::unix::io::{FromRawFd, AsRawFd};
// use std::error::Error;

use rustbox::{Color, InitOptions, InputMode};
// use grimmbox::GrimmBox;
use rustbox::Key;

const STDOUT: i32 = 1;
// const STDERR: i32 = 2;


fn main() {
    // Initialise log redirect
    let logfile = File::create("./somelog.log").unwrap();
    env_logger::init().unwrap();
    let redirect = Redirect::stderr(logfile);

    let initoptions = rustbox::InitOptions {
        input_mode: InputMode::Current,
        buffer_stderr: false,
    };

    let gb = match ::grimmbox::GrimmBox::init(initoptions) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };




    // Test log redirect
    error!("Hello, stdout rustbox!");
    drop(redirect);
}
