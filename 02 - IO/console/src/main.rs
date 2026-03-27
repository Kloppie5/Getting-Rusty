use std::io;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Critical error");

    println!("Input: {}", &input);
    println!("Input: {input}");

    let input: u32 = input.trim().parse().expect("NaN");

    println!("Input: {}", input);
    println!("Input: {}", &input);
    println!("Input: {input}");
}
