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
    let lines: Vec<&str> = contents.lines().collect();
    let num_lines: usize = lines.len();
    let mut sum_part_numbers: u32 = 0;
    for (iline, line) in contents.lines().enumerate() {
        let line_len = line.len();
        let mut istart: usize = line_len; // start index of a number, valid if < line_len
        let mut iend: usize = line_len;   // end index of a number, valid if < line_len
        // println!("\n{line}");
        for (ich, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                if istart >= line_len {
                    istart = ich;
                }
            }
            else {
                if istart < line_len {
                    iend = ich - 1;
                }
            }
            // println!("{ich}, {istart}, {iend}");
            if istart < line_len && iend < line_len {
                // There is a number at indices [istart, iend - 1]
                let mut is_part_number: bool = false;
                loop {
                    // 1. Check the current line
                    if istart > 0 && is_char_symbol(&line.chars().nth(istart - 1).unwrap()) {
                        is_part_number = true;
                        break;
                    }
                    if iend + 1 < line_len && is_char_symbol(&line.chars().nth(iend + 1).unwrap()) {
                        is_part_number = true;
                        break;
                    }
                    // 2. Check the previous line
                    if iline > 0 {
                        if check_for_symbols(&lines[iline - 1], istart, iend) {
                            is_part_number = true;
                            break;
                        }
                    }
                    // 3. Check the next line
                    if iline + 1 < num_lines {
                        if check_for_symbols(&lines[iline + 1], istart, iend) {
                            is_part_number = true;
                            break;
                        }
                    }
                    break;
                }
                let number_str = &line[istart..=iend];
                println!("({is_part_number}, {number_str})");
                if is_part_number {
                    let part_number: u32 = number_str.parse::<u32>().unwrap();
                    sum_part_numbers += part_number;
                }

                istart = line_len;
                iend = line_len;
            }
        }
    }
    sum_part_numbers
}

fn check_for_symbols(line:& str, istart: usize, iend: usize) -> bool {
    // Check the characters line[istart..=iend] if it contains any symbol
    let check_start_index: usize = if istart > 0 { istart - 1 } else { istart };
    let check_end_index: usize = if iend + 1 < line.len() { iend + 1 } else { iend };
    for check_index in check_start_index..=check_end_index {
        if is_char_symbol(&line.chars().nth(check_index).unwrap()) {
            return true;
        }
    }
    false
}

fn is_char_symbol(ch: &char) -> bool {
    if ch.is_numeric() {
        return false;
    }
    if *ch == '.' {
        return false;
    }
    true
}
