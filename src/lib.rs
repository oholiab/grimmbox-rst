#[macro_use]
extern crate log;
extern crate rustbox;
extern crate unicode_segmentation;

use self::rustbox::RustBox;
use self::rustbox::Color;
use unicode_segmentation::UnicodeSegmentation;

pub type GrimmBox = RustBox;

pub struct Corner {
    x: usize,
    y: usize,
    glyph: char,
}

// In case you want anything else to GrimmBox like a GrimmBox does
// (although mostly because it makes extending RustBox easier)
pub trait GrimmBoxes {
    fn render(&self);
    fn draw_box(&self, x: usize, y: usize, w: usize, h: usize, fg: Color, bg: Color);
    fn text_box(&self,
                x: usize,
                y: usize,
                w: usize,
                h: usize,
                fg: Color,
                bg: Color,
                title: &str,
                body: &str);
}

impl GrimmBoxes for GrimmBox {
    fn render(&self) {
        self.present();
    }
    fn draw_box(&self, x: usize, y: usize, w: usize, h: usize, fg: Color, bg: Color) {
        let tl = '┌';
        let tr = '┐';
        let bl = '└';
        let br = '┘';
        let bottom = '─';
        let side = '│';
        let corners: [Corner; 4] = [Corner {
                                        x: x,
                                        y: y,
                                        glyph: tl,
                                    },
                                    Corner {
                                        x: x + w,
                                        y: y,
                                        glyph: tr,
                                    },
                                    Corner {
                                        x: x,
                                        y: y + h,
                                        glyph: bl,
                                    },
                                    Corner {
                                        x: x + w,
                                        y: y + h,
                                        glyph: br,
                                    }];

        for xx in corners[0].x..corners[1].x {
            self.print_char(xx, corners[0].y, rustbox::RB_BOLD, fg, bg, bottom);
            self.print_char(xx, corners[2].y, rustbox::RB_BOLD, fg, bg, bottom);
        }
        for yy in corners[0].y..corners[2].y {
            self.print_char(corners[0].x, yy, rustbox::RB_BOLD, fg, bg, side);
            self.print_char(corners[1].x, yy, rustbox::RB_BOLD, fg, bg, side);
        }

        for c in corners.iter() {
            self.print_char(c.x, c.y, rustbox::RB_BOLD, fg, bg, c.glyph);
        }

    }
    fn text_box(&self,
                x: usize,
                y: usize,
                w: usize,
                h: usize,
                fg: Color,
                bg: Color,
                title: &str,
                body: &str) {
        self.draw_box(x, y, w, h, fg, bg);
        let max_width = w - 2;
        let print_title = shorten_string(title, max_width);

        self.print(x + 1, y, rustbox::RB_BOLD, fg, bg, &print_title);
        let mut y_print = y + 1;
        for line in reflow_text2(&body, w - 2, h - 2) {
            self.print(x + 1, y_print, rustbox::RB_NORMAL, fg, bg, &line);
            y_print += 1;
        }
    }
}

fn shorten_string(string: &str, len: usize) -> String {
    // I wanted to do this by returning a slice of the previous string but unicode lol
    let graphemes = UnicodeSegmentation::graphemes(string, true);
    let mut return_string = "".to_string();
    let mut acc = 0;
    for g in graphemes {
        if acc <= len {
            return_string.push_str(g);
            acc += 1;
        } else {
            break;
        }
    }
    return return_string;
}

fn reflow_text(string: &str, width: usize, height: usize) -> Vec<String> {
    let mut text = vec![];
    let mut graphemes = UnicodeSegmentation::graphemes(string, true);
    'all: for row in 0..height + 1 {
        let mut line = "".to_string();
        for col in 0..width + 1 {
            match graphemes.next() {
                Some("\n") => break, 
                Some(x) => line.push_str(x),
                None => {
                    text.push(line);
                    break 'all;
                }
            }
        }
        text.push(line);
    }
    return text;
}

fn reflow_text2(string: &str, width: usize, height: usize) -> Vec<String> {
    let mut text = vec![];
    let mut graphemes = UnicodeSegmentation::graphemes(string, true);
    let mut buf = "".to_string();
    'all: for row in 0..height + 1 {
        let mut line = "".to_string();
        while line.len() < width + 1 {
            match graphemes.next() {
                Some("\n") => break, 
                Some(" ") => {
                    if line.len() + buf.len() < width {
                        line.push_str(&buf);
                    } else {
                        buf.push_str(" ");
                        break;
                    }
                    if line.len() < width {
                        line.push_str(" ");
                    }
                    buf = "".to_string();
                }
                Some(x) => buf.push_str(x),
                None => {
                    text.push(line);
                    break 'all;
                }
            }
        }
        text.push(line);
    }
    return text;
}
