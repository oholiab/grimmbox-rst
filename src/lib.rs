#[macro_use]
extern crate log;
extern crate rustbox;

use self::rustbox::RustBox;
use self::rustbox::Color;

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

        self.print(x + 1, y, rustbox::RB_BOLD, fg, bg, print_title);
    }
}

fn shorten_string(string: &str, len: usize) -> &str {
    if string.len() > len {
        return string.split_at(len).0;
    } else {
        return string;
    };
}
