use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	let file_path = &args[1];
    println!("Reading file {file_path}");

	let contents = fs::read_to_string(file_path)
		.expect("File {file_path} is not valid");
	let result1: u32 = part1(&contents);
	println!("{result1}");
}

fn part1(contents: &str) -> u32 {
	const RADIX: u32 = 10;
	let mut i1: usize; // index to the first digit in the line
	let mut i2: usize; // index to the last digit in the line
	let mut n1: u32;
	let mut n2: u32;
	let mut result: u32 = 0;
	for line in contents.lines() {
		// println!("{line}");
		// Assume numbers always exist in each line
		i1 = line.find(|c: char| c.is_numeric()).unwrap();
		i2 = line.rfind(|c: char| c.is_numeric()).unwrap();
		n1 = line.chars().nth(i1).unwrap()
			.to_digit(RADIX).unwrap();
		n2 = line.chars().nth(i2).unwrap()
			.to_digit(RADIX).unwrap();
		result += n1*10 + n2;
	}
	result
}
