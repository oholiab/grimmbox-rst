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
        let tl = '.';
        let tr = '.';
        let bl = '.';
        let br = '.';
        let bottom = '-';
        let side = '|';
        let corners: [[usize; 2]; 4] = [[x, y], [x + w, y], [x, y + h], [x + w, y + h]];

        // 1 to 2
        (corners[0][X]..corners[1][X])
            .map(|xx| self.print_char(xx, corners[0][Y], rustbox::RB_BOLD, fg, bg, bottom));
        (1..10).map(|xx| self.print_char(xx, 4, rustbox::RB_BOLD, fg, bg, tl));
        error!("did it work?");
        (1..10).map(|xx| error!("{}", xx));

        // 3 to 4
        // 1 to 3
        // 2 to 4

        for c in corners.iter() {
            self.print_char(c[X], c[Y], rustbox::RB_BOLD, fg, bg, tl);
        }

    }
}
