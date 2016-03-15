extern crate rustbox;

use self::rustbox::RustBox;

// Needs putting back in:
pub type GrimmBox = RustBox;

pub trait GrimmBoxes {
    fn render(&self);
}

impl GrimmBoxes for GrimmBox {
    fn render(&self) {
        self.present();
    }
}
