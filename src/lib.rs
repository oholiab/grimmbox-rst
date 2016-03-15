#[macro_use]
extern crate log;
extern crate rustbox;

use self::rustbox::RustBox;
use self::rustbox::Color;

// Array indexing for later
const X: usize = 0;
const Y: usize = 1;

// Needs putting back in:
pub type GrimmBox = RustBox;

pub trait GrimmBoxes {
    fn render(&self);
    fn drawBox(&self, x: usize, y: usize, w: usize, h: usize, fg: Color, bg: Color);
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
        let corners: [[usize; 2]; 4] = [[x, y], [x + w, y], [x, y + h], [x + w, y + h]];

        for xx in (corners[0][X]..corners[1][X]) {
            self.print_char(xx, corners[0][Y], rustbox::RB_BOLD, fg, bg, bottom);
            self.print_char(xx, corners[2][Y], rustbox::RB_BOLD, fg, bg, bottom);
        }
        for yy in (corners[0][Y]..corners[2][Y]) {
            self.print_char(corners[0][X], yy, rustbox::RB_BOLD, fg, bg, side);
            self.print_char(corners[1][X], yy, rustbox::RB_BOLD, fg, bg, side);
        }

        for c in corners.iter() {
            self.print_char(c[X], c[Y], rustbox::RB_BOLD, fg, bg, tl);
        }

    }
}
