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
}

fn part1(contents: &str) -> u32 {
	const NUM_RED: u32 = 12;
	const NUM_GREEN: u32 = 13;
	const NUM_BLUE: u32 = 14;

	let mut is_possible: bool;
	let mut sum_id: u32 = 0;
	for line in contents.lines() {
		// Each line is of the form
		// Game X: i1 blue, i2 red, i3 green; ...
		let v: Vec<&str> = line.split(":").collect();
		let game_id_str: &str = &v[0][5..]; // v[0] is of the form "Game X"
		// println!("{game_id_str}");
		is_possible = analyze_game_samples(&v[1], NUM_RED, NUM_GREEN, NUM_BLUE);
		if is_possible {
			sum_id += game_id_str.parse::<u32>().unwrap();
		}
	}
	sum_id
}

fn analyze_game_samples(samples: &str, num_red: u32, num_green: u32, num_blue: u32) -> bool {
	for sample in samples.trim().split(";") {
		for set in sample.trim().split(",") {
			let (number_str, color_str) = set.trim().split_once(" ").unwrap();
			let num_cubes: u32 = number_str.parse::<u32>().unwrap();
			if color_str == "red" && num_cubes > num_red {
				return false;
			}
			else if color_str == "green" && num_cubes > num_green {
				return false;
			}
			else if color_str == "blue" && num_cubes > num_blue {
				return false;
			}
		}
	}
	true
}
