#[macro_use]
extern crate log;
extern crate rustbox;

use self::rustbox::RustBox;
use self::rustbox::Color;

// Array indexing for later
const X: usize = 0;
const Y: usize = 1;

pub type GrimmBox = RustBox;

pub struct Corner {
    X: usize,
    Y: usize,
    Glyph: char,
}

// In case you want anything else to GrimmBox like a GrimmBox does
// (although mostly because it makes extending RustBox easier)
pub trait GrimmBoxes {
    fn render(&self);
    fn drawBox(&self, x: usize, y: usize, w: usize, h: usize, fg: Color, bg: Color);
    fn textBox(&self,
               x: usize,
               y: usize,
               w: usize,
               h: usize,
               fg: Color,
               bg: Color,
               title: String,
               body: String);
}

impl GrimmBoxes for GrimmBox {
    fn render(&self) {
        self.present();
    }
    fn drawBox(&self, x: usize, y: usize, w: usize, h: usize, fg: Color, bg: Color) {
        let tl = '┌';
        let tr = '┐';
        let bl = '└';
        let br = '┘';
        let bottom = '─';
        let side = '│';
        let corners: [Corner; 4] = [Corner {
                                        X: x,
                                        Y: y,
                                        Glyph: tl,
                                    },
                                    Corner {
                                        X: x + w,
                                        Y: y,
                                        Glyph: tr,
                                    },
                                    Corner {
                                        X: x,
                                        Y: y + h,
                                        Glyph: bl,
                                    },
                                    Corner {
                                        X: x + w,
                                        Y: y + h,
                                        Glyph: br,
                                    }];

        for xx in corners[0].X..corners[1].X {
            self.print_char(xx, corners[0].Y, rustbox::RB_BOLD, fg, bg, bottom);
            self.print_char(xx, corners[2].Y, rustbox::RB_BOLD, fg, bg, bottom);
        }
        for yy in corners[0].Y..corners[2].Y {
            self.print_char(corners[0].X, yy, rustbox::RB_BOLD, fg, bg, side);
            self.print_char(corners[1].X, yy, rustbox::RB_BOLD, fg, bg, side);
        }

        for c in corners.iter() {
            self.print_char(c.X, c.Y, rustbox::RB_BOLD, fg, bg, c.Glyph);
        }

    }
    fn textBox(&self,
               x: usize,
               y: usize,
               w: usize,
               h: usize,
               fg: Color,
               bg: Color,
               title: String,
               body: String) {
        self.drawBox(x, y, w, h, fg, bg);
    }
}
