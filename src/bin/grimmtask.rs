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
                        "e (1), but which allows backward movement in the file as well as \
                         forward movement.  Also, less does not have to read the entire input \
                         file before starting, so with large input files it starts up faster \
                         than text editors like vi (1).  Less uses termcap  (or  terminfo  on  \
                         some systems),  so  it can run on a variety of terminals.  There is \
                         even limited support for hardcopy terminals.  (On a hardcopy terminal, \
                         lines which should be printed at the top of the screen are");
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
            Ok(rustbox::Event::ResizeEvent(_, _)) => {
                gb.clear();
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }

    // Test log redirect
    drop(redirect);
}
