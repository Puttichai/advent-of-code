use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("Reading file {file_path}");

    let contents: String = fs::read_to_string(file_path)
        .expect("File {file_path} is not valid");
    let lines: Vec<&str> = contents.lines().collect();
    
    let times: Vec<u64> = extract_numbers(&lines[0]);
    let distances: Vec<u64> = extract_numbers(&lines[1]);
    println!("times={:?}", times);
    println!("distances={:?}", distances);
    let result1: u64 = part1(&times, &distances);
    println!("result1 = {result1}");
}

fn extract_numbers(line: &str) -> Vec<u64> {
    line[
        (line.chars().position(|c| c == ':').unwrap() + 1)..line.len()
    ]
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|time_str| {
            time_str.parse::<u64>().unwrap()
        })
        .collect()
}

fn part1(times: &Vec<u64>, distances: &Vec<u64>) -> u64 {
    let mut result: u64 = 1;
    for (ittime, itdistance) in times.iter().zip(distances.iter()) {
        let num_ways = compute_num_ways(*ittime, *itdistance);
        println!("time={}; distance={}; ways={num_ways}", *ittime, *itdistance);
        result = result * num_ways;
    }
    result
}

// Given the total time T and the total distance D, compute the number
// of possible values of hold time (H) so that in the end, the
// distance D could be covered with no more than time T - H.
fn compute_num_ways(time: u64, distance: u64) -> u64 {
    let t = time as f64;
    let d = distance as f64;
    let det: f64 = (t*t - 4_f64*d).sqrt();
    let max_hold_time: f64 = (t + det) * 0.5_f64;
    let min_hold_time: f64 = (t - det) * 0.5_f64;

    let max_hold_time_int = max_hold_time.floor() as u64;
    let min_hold_time_int = min_hold_time.ceil() as u64;
    if max_hold_time.fract() == 0.0 {
        // Both max and min hold time are ints
        if (min_hold_time_int + 1) > (max_hold_time_int - 1) {
            return 0;
        }
        else {
            return (max_hold_time_int - 1) - (min_hold_time_int + 1) + 1;
        }
    }
    max_hold_time_int - min_hold_time_int + 1
}
