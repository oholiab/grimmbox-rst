extern crate rustbox;

use rustbox::RustBox;

type Grimmbox = RustBox;

trait Grimmboxes {
    fn render(&self);
}

impl Grimmboxes for Grimmbox {
    fn render(&self) {
        self.present();
    }
}
