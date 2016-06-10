#[macro_use]
extern crate log;
extern crate env_logger;

extern crate gag;
extern crate rustbox;
extern crate grimmbox;

use gag::Redirect;
use std::fs::File;

use rustbox::{Color, InitOptions, InputMode};
use grimmbox::{GrimmBox, GrimmBoxes};
use rustbox::Key;

fn main() {
    // Initialise log redirect
    let logfile = File::create("./somelog.log").unwrap();
    env_logger::init().unwrap();
    let redirect = Redirect::stderr(logfile);

    let initoptions = InitOptions {
        input_mode: InputMode::Current,
        buffer_stderr: false,
    };

    let gb = match GrimmBox::init(initoptions) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    gb.textBox(1,
               1,
               10,
               5,
               Color::White,
               Color::Black,
               "Hello, world!",
               "stuff");
    gb.render();
    loop {
        match gb.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Ctrl('c') | Key::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    // Test log redirect
    error!("Hello, stdout rustbox!");
    drop(redirect);
}
