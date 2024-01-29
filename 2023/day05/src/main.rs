use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("Reading file {file_path}");

    let contents: String = fs::read_to_string(file_path)
        .expect("File {file_path} is not valid");

    let seeds: Vec<u64> = extract_seeds(&contents);
    let input_maps: Vec<Vec<[u64; 3]>> = extract_maps(&contents);
    let result1: u64 = part1(&seeds, &input_maps);
    println!("result1 = {result1}");

    let result2: u64 = part2(&seeds, &input_maps);
    println!("result2 = {result2}");
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
                map.sort_by_key(|k| k[1]); // sort by source indices
                maps.push(map.clone());
                map.clear();
                started = false;
                continue;
            }
            let v: Vec<&str> = line.trim().split_whitespace().collect();
            assert!(v.len() == 3);
            map.push( [v[0].parse::<u64>().unwrap(),    // dest
                       v[1].parse::<u64>().unwrap(),    // source
                       v[2].parse::<u64>().unwrap()] ); // len
        }
    }
    map.sort_by_key(|k| k[1]); // sort by source indices
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

fn part2(seeds: &Vec<u64>, maps: &Vec<Vec<[u64; 3]>>) -> u64 {
    // Prepare the input
    let mut input_ranges: Vec<[u64; 2]> = vec![];
    for iseed in 0..seeds.len()/2 {
        input_ranges.push([seeds[2*iseed], seeds[2*iseed + 1]]);
    }
    // println!("input_ranges={:?}", input_ranges);

    let mut map_index: usize = 0;
    for map in maps {
        // println!("map_index={map_index}, map={:?}", map);
        map_index += 1;
        let mut output_ranges: Vec<[u64; 2]> = vec![];
        for input_range in input_ranges {
            output_ranges.append(
                &mut convert_source_to_dest(&input_range, &map)
            );
        }
        // println!("output_ranges={:?}\n", output_ranges);
        input_ranges = output_ranges; // update the input for the next map
    }

    input_ranges.sort_by_key(|k| k[0]); // sort by the start index
    // println!{"final_ranges={:?}", input_ranges}
    input_ranges[0][0] // return the minimum start index
}

fn convert_source_to_dest(source_range: &[u64; 2], map: &Vec<[u64; 3]>) -> Vec<[u64; 2]> {
    let mut output_ranges: Vec<[u64; 2]> = vec![];

    let mut input_ranges: Vec<[u64; 2]> = vec![];
    input_ranges.push(
        [source_range[0], // start_index
         source_range[0] + source_range[1] - 1] // end_index
    );
    for dest_source_len in map {
        let dest: u64 = dest_source_len[0];
        let source: u64 = dest_source_len[1];
        let len: u64 = dest_source_len[2];

        let input_start: u64 = input_ranges[0][0];
        let input_end: u64 = input_ranges[0][1];
        if input_end < source {
            output_ranges.push(
                [input_start, input_end - input_start + 1]
            );
            input_ranges.pop();
            break;
        }
        if input_start > source + len - 1 {
            continue;
        }

        if input_start < source {
            output_ranges.push(
                [input_start, source - input_start]
            );

            if input_end <= source + len - 1 {
                output_ranges.push(
                    [dest, (input_end - source + 1)]
                );
                input_ranges.pop();
                break;
            }
            else {
                output_ranges.push(
                    [dest, len]
                );
                input_ranges[0][0] = source + len;
                continue;
            }
        }
        else {
            if input_end <= source + len - 1 {
                output_ranges.push(
                    [dest + (input_start - source), input_end - input_start + 1]
                );
                input_ranges.pop();
                break;
            }
            else {
                output_ranges.push(
                    [dest + (input_start - source), (source + len - 1) - input_start + 1]
                );
                input_ranges[0][0] = source + len;
                continue;
            }
        }
    }
    while input_ranges.len() > 0 {
        let input_range = input_ranges.pop().unwrap();
        output_ranges.push(
            [input_range[0], input_range[1] - input_range[0] + 1]
        );
    }
    output_ranges
}
