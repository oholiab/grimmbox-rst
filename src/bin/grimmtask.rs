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
    info!("Starting up");
    let tasks_per_window = 5;
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

    loop {
        let box_inside_height = gb.height() - 3;
        let box_inside_width = gb.width() - 3;
        let task_height = box_inside_height / tasks_per_window;
        gb.text_box(0,
                    0,
                    gb.width() - 1,
                    gb.height() - 1,
                    Color::White,
                    Color::Black,
                    "GRIMMT⛧ SK",
                    "");
        for i in 0..tasks_per_window {
            gb.text_box(1,
                        1 + i * task_height,
                        box_inside_width,
                        task_height - 1,
                        Color::White,
                        Color::Black,
                        &format!("Task {}", i),
                        "some crap");
        }

        gb.render();
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
    drop(redirect);
}
