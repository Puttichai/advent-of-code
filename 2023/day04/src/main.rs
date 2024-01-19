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

    let result2: u32 = part2(&contents);
    println!("result2 = {result2}");
}

fn part1(contents: &str) -> u32 {
    let mut total_score: u32 = 0;
    for line in contents.lines() {
        let num_matches: u32 = check_num_matches(line);
        if num_matches > 0 {
            total_score += 2_u32.pow(num_matches - 1);
        }
    }
    total_score
}

fn check_num_matches(line: &str) -> u32 {
    let v1: Vec<&str> = line.split(":").collect();
    let v2: Vec<&str> = v1[1].split("|").collect();
    let winning_numbers_str: Vec<&str> = v2[0].trim().split_whitespace().collect();
    let own_numbers_str: Vec<&str> = v2[1].trim().split_whitespace().collect();
    let mut num_matches: u32 = 0;
    for own_number_str in own_numbers_str {
        if winning_numbers_str.contains(&own_number_str) {
            num_matches += 1;
        }
    }
    num_matches
}

fn part2(contents: &str) -> u32 {
    let lines: Vec<&str> = contents.lines().collect();
    let num_lines: usize = lines.len();
    let mut num_cards: Vec<u32> = vec![1; num_lines]; // each numbered card has one original copy
    for (icard, line) in lines.iter().enumerate() {
        let num_matches: u32 = check_num_matches(line);
        let num_current_card: u32 = num_cards[icard];
        if num_matches > 0 {
            for next_card_index in (icard + 1)..=(icard + (num_matches as usize)) {
                num_cards[next_card_index] += num_current_card;
            }
        }
    }
    num_cards.iter().sum()
}
