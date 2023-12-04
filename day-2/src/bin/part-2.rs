use std::fs::File;
use std::io::Read;
fn main() {
    let file = File::open("input.txt");
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
