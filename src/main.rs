
use std::env;
use std::fs::File;
mod rank;
use rank::Test;
fn main() {
    println!("Hello, world!");

    let mut re = env::temp_dir();
    re.push("foo.txt");
    println!("{:?}", re);

}
