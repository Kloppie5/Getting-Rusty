use std::env;
use std::fs;

fn main() {
    println!("The current directory is {}", env::current_dir().expect("No").display());

    let data = fs::read_to_string("./data/somefile.txt")
        .expect("Should be able to read file");

    println!("{}", data);

    let data = fs::read("./data/somebinaryfile")
        .expect("Should be able to read file");

    println!("{:X?}", data);
}
