extern crate grimmbox;
use grimmbox::*;

fn main() {
    let msg = "Hello, world!";
    println!("{}", msg.split_at(3).0);
}
