use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("Reading file {file_path}");

    let contents: String = fs::read_to_string(file_path)
        .expect("File {file_path} is not valid");

    let seeds: Vec<u64> = extract_seeds(&contents);
    println!("There are {} seeds", seeds.len());
    let input_maps: Vec<Vec<[u64; 3]>> = extract_maps(&contents);
    println!("There are {} maps", input_maps.len());
    let result1: u64 = part1(&seeds, &input_maps);
    println!("result1 = {result1}");

    // // Part2: same as part 1 but different inputs
    // TODO: Cannot use the same naive method for part 2 or the machine will crash
    // let seeds2: Vec<u64> = extract_seeds2(&contents);
    // println!("There are actually {} seeds", seeds2.len());
    // let result2: u64 = part1(&seeds2, &input_maps);
    // println!("result2 = {result2}");
}

fn extract_seeds(contents: &str) -> Vec<u64> {
    let first_line: &str = &contents[
        (contents.chars().position(|c| c == ':').unwrap() + 1)..(contents.chars().position(|c| c == '\n').unwrap())
    ]
        .trim();

    let seed_strs: Vec<&str> = first_line.split_whitespace().collect();
    seed_strs.into_iter()
        .map(|seed_str| {
            seed_str.parse::<u64>().unwrap()
        })
        .collect()
}

fn extract_seeds2(contents: &str) -> Vec<u64> {
    let mut seeds: Vec<u64> = vec![];
    let lines: Vec<&str> = contents.lines().collect();
    let line: &str = lines[0];
    let v1: Vec<&str> = line.trim().split(":").collect();
    let v2: Vec<&str> = v1[1].trim().split_whitespace().collect();
    let num_pairs: usize = v2.len()/2;
    for ipair in 0..num_pairs {
        let seed_start: u64 = v2[2*ipair + 0].parse::<u64>().unwrap();
        let seed_range_len: u64 = v2[2*ipair + 1].parse::<u64>().unwrap();
        for seed in seed_start..(seed_start + seed_range_len) {
            seeds.push(seed);
        }
    }
    seeds
}

fn extract_maps(contents: &str) -> Vec<Vec<[u64; 3]>> {
    let mut maps: Vec<Vec<[u64; 3]>> = vec![];
    let mut started: bool = false;
    let mut map: Vec<[u64; 3]> = vec![];
    for line in contents.lines() {
        if !started && line.contains(":") && !line.contains("seeds:") {
            started = true;
            continue;
        }
        if started {
            if line.len() == 0 {
                maps.push(map.clone());
                map.clear();
                started = false;
                continue;
            }
            let v: Vec<&str> = line.trim().split_whitespace().collect();
            assert!(v.len() == 3);
            map.push( [v[0].parse::<u64>().unwrap(),
                       v[1].parse::<u64>().unwrap(),
                       v[2].parse::<u64>().unwrap()] );
        }
    }
    maps.push(map.clone());
    maps
}

fn part1(seeds: &Vec<u64>, maps: &Vec<Vec<[u64; 3]>>) -> u64 {
    let mut input_indices: Vec<u64> = seeds.clone();
    for map in maps {
        let mut output_indices: Vec<u64> = vec![];
        for index in input_indices {
            let dest: u64 = find_destination(&index, map);
            output_indices.push(dest);
        }
        input_indices = output_indices;
    }
    *input_indices.iter().min().unwrap()
}

fn find_destination(index: &u64, map: &Vec<[u64; 3]>) -> u64 {
    for dest_source_len in map {
        let dest: u64 = dest_source_len[0];
        let source: u64 = dest_source_len[1];
        let len: u64 = dest_source_len[2];
        if source <= *index && *index < source + len {
            return dest + (*index - source);
        }
    }
    *index
}
