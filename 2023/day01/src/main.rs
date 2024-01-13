use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	let file_path = &args[1];
    println!("Reading file {file_path}");

	let contents = fs::read_to_string(file_path)
		.expect("File {file_path} is not valid");
	let result1: u32 = part1(&contents);
	println!("result1 = {result1}");
	let result2: u32 = part2(&contents);
	println!("result2 = {result2}");
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

fn part2(contents: &str) -> u32 {
	let (mut i1, mut i2): (Option<usize>, Option<usize>);
	let (mut r1, mut r2): (Option<u32>, Option<u32>);
	let mut result: u32 = 0;
	let mut value: u32; // temporary variable
	for line in contents.lines() {
		// Forward
		(i1, r1) = get_index_and_value_word(line);
		(i2, r2) = get_index_and_value_digit(line);
		if i1.is_some() && i2.is_some() {
			if i1 < i2 {
				value = r1.unwrap();
			}
			else {
				value = r2.unwrap();
			}
		}
		else if i1.is_some() {
			value = r1.unwrap();
		}
		else {
			value = r2.unwrap();
		}
		result += value*10;

		// Backward
		(i1, r1) = rget_index_and_value_word(line);
		(i2, r2) = rget_index_and_value_digit(line);
		if i1.is_some() && i2.is_some() {
			if i1 > i2 {
				value = r1.unwrap();
			}
			else {
				value = r2.unwrap();
			}
		}
		else if i1.is_some() {
			value = r1.unwrap();
		}
		else {
			value = r2.unwrap();
		}
		result += value;		
	}
	result
}

fn get_index_and_value_word(line: &str) -> (Option<usize>, Option<u32>) {
	let numbers: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
	let mut index: usize = line.len();
	let mut result: u32 = 0;
	let mut i1: usize;
	for (cur_index, strnumber) in numbers.iter().enumerate() {
		i1 = match line.find(strnumber) {
			Some(i) => i,
			None => continue,
		};
		if i1 < index {
			index = i1;
			result = (cur_index as u32) + 1;
		}
	}
	if result == 0 {
		(None, None)
	}
	else {
		(Some(index), Some(result))
	}
}

fn rget_index_and_value_word(line: &str) -> (Option<usize>, Option<u32>) {
	let numbers: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
	let mut index: usize = 0;
	let mut result: u32 = 0;
	let mut has_result = false;
	let mut i1: usize;
	for (cur_index, strnumber) in numbers.iter().enumerate() {
		i1 = match line.rfind(strnumber) {
			Some(i) => i + strnumber.len() - (1 as usize), // index of the last letter in the word
			None => continue,
		};
		has_result = true;
		if i1 > index {
			index = i1;
			result = (cur_index as u32) + 1;
		}
	}
	if !has_result {
		(None, None)
	}
	else {
		(Some(index), Some(result))
	}
}

fn get_index_and_value_digit(line: &str) -> (Option<usize>, Option<u32>) {
	const RADIX: u32 = 10;
	match line.find(|c: char| c.is_numeric()) {
		Some(index) => {
			let value: u32 = line.chars().nth(index).unwrap()
				.to_digit(RADIX).unwrap();
			(Some(index), Some(value))
		},
		None => (None, None),
	}
}

fn rget_index_and_value_digit(line: &str) -> (Option<usize>, Option<u32>) {
	const RADIX: u32 = 10;
	match line.rfind(|c: char| c.is_numeric()) {
		Some(index) => {
			let value: u32 = line.chars().nth(index).unwrap()
				.to_digit(RADIX).unwrap();
			(Some(index), Some(value))
		},
		None => (None, None),
	}
}
