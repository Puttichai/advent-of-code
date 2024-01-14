use std::env;
use std::fs;

fn main() {
    let args: Vect<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("File {file_path} is not valid");
    let result1: u32 = part1(&contents);
    println("result1 = {result1}");
}

fn part1(contents: &str) -> u32 {
    0
}
