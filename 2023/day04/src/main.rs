use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("Reading file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("File {file_path} is not valid");
    let result1: u32 = part1(&contents);
    println!("result1 = {result1}");
}

fn part1(contents: &str) -> u32 {
    let mut total_score: u32 = 0;
    for line in contents.lines() {
        let score: u32 = compute_line_score(line);
        total_score += score;
    }
    total_score
}

fn compute_line_score(line: &str) -> u32 {
    let v1: Vec<&str> = line.split(":").collect();
    let v2: Vec<&str> = v1[1].split("|").collect();
    let winning_numbers_str: Vec<&str> = v2[0].trim().split_whitespace().collect();
    let own_numbers_str: Vec<&str> = v2[1].trim().split_whitespace().collect();
    let mut num_matched: u32 = 0;
    for own_number_str in own_numbers_str {
        if winning_numbers_str.contains(&own_number_str) {
            num_matched += 1;
        }
    }
    if num_matched > 0 {
        2_u32.pow(num_matched - 1)
    }
    else {
        0
    }
}
