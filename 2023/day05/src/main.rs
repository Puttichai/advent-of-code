use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("Reading file {file_path}");

    let contents: String = fs::read_to_string(file_path)
        .expect("File {file_path} is not valid");

    let seeds: Vec<u64> = extract_seeds(&contents);
    let seed_to_soil_map: Vec<[u64; 3]> = extract_seed_to_soil_map(&contents);
    let result1: u64 = part1(&seeds, &seed_to_soil_map);
    println!("result1 = {result1}");
}

fn extract_seeds(contents: &str) -> Vec<u64> {
    let mut seeds: Vec<u64> = vec![];
    let lines: Vec<&str> = contents.lines().collect();
    let line: &str = lines[0];
    let v1: Vec<&str> = line.trim().split(":").collect();
    let v2: Vec<&str> = v1[1].trim().split_whitespace().collect();
    for seed_str in v2 {
        seeds.push( seed_str.parse::<u64>().unwrap() );
    }
    seeds
}

fn extract_seed_to_soil_map(contents: &str) -> Vec<[u64; 3]> {
    let mut seed_to_soil_map: Vec<[u64; 3]> = vec![];
    let mut started: bool = false;
    for line in contents.lines() {
        if !started && line.contains("seed-to-soil") {
            started = true;
            continue;
        }
        if started {
            if line.len() == 0 {
                break;
            }
            let v: Vec<&str> = line.trim().split_whitespace().collect();
            assert!(v.len() == 3);
            seed_to_soil_map.push( [v[0].parse::<u64>().unwrap(),
                                    v[1].parse::<u64>().unwrap(),
                                    v[2].parse::<u64>().unwrap()] );
        }
    }
    seed_to_soil_map
}

fn part1(seeds: &Vec<u64>, seed_to_soil_map: &Vec<[u64; 3]>) -> u64 {
    let mut min_dest: Option<u64> = None;
    for seed in seeds {
        let dest: u64 = find_seed_destination(seed, seed_to_soil_map);
        if min_dest.is_none() || dest < min_dest.unwrap() {
            min_dest = Some(dest);
        }
    }
    assert!( min_dest.is_some() );
    min_dest.unwrap()
}

fn find_seed_destination(seed: &u64, seed_to_soil_map: &Vec<[u64; 3]>) -> u64 {
    for dest_source_len in seed_to_soil_map {
        let dest: u64 = dest_source_len[0];
        let source: u64 = dest_source_len[1];
        let len: u64 = dest_source_len[2];
        if source <= *seed && *seed < source + len {
            return dest + (*seed - source);
        }
    }
    *seed
}
