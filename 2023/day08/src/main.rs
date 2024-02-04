use std::env;
use std::fs;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("Reading file {file_path}");

    let contents: String = fs::read_to_string(file_path)
        .expect("File {file_path} is not valid");

    let instructions: Vec<u64> = contents.lines()
        .collect::<Vec<_>>()[0]
        .chars()
        .map(|ch| {
            return if ch == 'L' {0_u64} else {1_u64};
        })
        .collect();
    println!("instructions={:?}", instructions);
    let network: HashMap<&str, (&str, &str)> = extract_network(&contents);

    let now1 = Instant::now();
    let result1: u64 = part1(&instructions, &network);
    let elapsed1 = now1.elapsed();
    println!("result1 = {result1} computed in {:?}", elapsed1);

    let now2 = Instant::now();
    let result2: u64 = part2(&instructions, &network);
    let elapsed2 = now2.elapsed();
    println!("result2 = {result2} computed in {:?}", elapsed2);
}

fn extract_network(contents: &str) -> HashMap<&str, (&str, &str)> {
    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in contents.lines() {
        if !line.contains("=") {
            continue;
        }
        let cur_node: &str = &line[0..3];
        let pos1: usize = line.chars().position(|c| c == '(').unwrap();
        let left_node: &str = &line[
            (pos1 + 1)..(pos1 + 4)
        ];
        let pos2: usize = line.chars().position(|c| c == ',').unwrap();
        let right_node: &str = &line[
            (pos2 + 2)..(pos2 + 5)
        ];
        network.insert(
            cur_node,
            (left_node, right_node),
        );
    }
    network
}

fn part1(instructions: &Vec<u64>, network: &HashMap<&str, (&str, &str)>) -> u64 {
    let dest_node: &str = "ZZZ";
    let mut current_node: &str = "AAA";
    let mut num_steps: u64 = 0;
    let mut current_instruction_index: usize = 0;
    let num_instructions: usize = instructions.len();
    loop {
        num_steps += 1;
        match network.get_key_value(current_node) {
            Some(key_value) => {
                let choices: &(&str, &str) = key_value.1;
                let next_node: &str = if instructions[current_instruction_index] == 0 { choices.0 } else { choices.1 };
                // println!("step {num_steps}: current node = {current_node}, next node = {next_node}");
                if next_node == dest_node {
                    break;
                }
                current_node = next_node;
                current_instruction_index += 1_usize;
                if current_instruction_index >= num_instructions {
                    current_instruction_index = 0_usize;
                }
            },
            None => {
                println!("[part1] Cannot find entry {current_node} in the network so terminating.");
                return 0;
            },
        }
    }
    num_steps
}

fn part2(instructions: &Vec<u64>, network: &HashMap<&str, (&str, &str)>) -> u64 {
    let mut current_state: Vec<&str> = vec![];
    for key in network.keys() {
        if key.chars().last().unwrap() == 'A' {
            current_state.push(key);
        }
    }    
    current_state.sort();
    
    // Assume that going from a node '--A' to a node '--Z' consumes the entire instructions N times.
    let mut num_cycles: Vec<u64> = vec![];
    let num_instructions: usize = instructions.len();

    for node in &current_state {
        let mut current_instruction_index: usize = 0;
        let mut num_steps: u64 = 0;
        let mut current_node: &str = node;
        loop {
            num_steps += 1;
            let current_instruction: u64 = instructions[current_instruction_index];
            let choices: &(&str, &str) = network.get_key_value(current_node).unwrap().1;
            let next_node: &str = if current_instruction == 0 {choices.0} else {choices.1};
            if next_node.chars().last().unwrap() == 'Z' {
                break;
            }

            current_node = next_node;
            current_instruction_index += 1_usize;
            if current_instruction_index >= num_instructions {
                current_instruction_index = 0;
            }
        }
        num_cycles.push(num_steps / (num_instructions as u64));
    }
    println!("num_cycles={:?}", num_cycles);
    
    num_cycles
        .iter()
        .fold(num_cycles[0],
              |acc, num_cycle| num_integer::lcm(acc, *num_cycle)) * (num_instructions as u64)
}
