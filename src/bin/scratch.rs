fn main() {
    let msg = "Hello, world!";
    let msg2 = "Hello w⛧rld!";
    println!("{} has length {}", msg, msg.bytes().count());
    println!("{} has length {}", msg2, msg2.bytes().count());
}
