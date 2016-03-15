extern crate rustbox;

use self::rustbox::RustBox;


// Needs putting back in:
// pub type GrimmBox = RustBox;

pub struct GrimmBox;

trait Test {
    fn present(&self);
}

trait GrimmBoxes {
    fn render(&self);
}

impl GrimmBoxes for GrimmBox {
    fn render(&self) {
        self.present();
    }
}

impl Test for GrimmBox {
    fn present(&self) {
        info!("presenting");
    }
}
