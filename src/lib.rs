#[macro_use]
extern crate log;
extern crate rustbox;

use self::rustbox::RustBox;

// Needs putting back in:
pub type GrimmBox = RustBox;

pub trait GrimmBoxes {
    fn render(&self);
    fn drawBox(&self, x: i32, y: i32, w: i32, h: i32);
}

impl GrimmBoxes for GrimmBox {
    fn render(&self) {
        self.present();
    }
    fn drawBox(&self, x: i32, y: i32, w: i32, h: i32) {}
}
