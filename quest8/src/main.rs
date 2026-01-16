use std::fs;

use itertools::Itertools;

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => println!("Test 1: {}", part1(&input, 8)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok(input) => println!("Input 1: {}", part1(&input, 32)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/test2.txt") {
        Ok(input) => println!("Test 2: {}", part2(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input2.txt") {
        Ok(input) => println!("Input 2: {}", part2(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn extract_input_from_file(file_path: &str) -> Result<Vec<u16>, String> {
    fs::read_to_string(file_path)
        .map_err(|e| e.to_string())?
        .trim()
        .split(',')
        .map(|n| {
            n.parse::<u16>()
                .map_err(|e| format!("Couldn't parse number: {n} - {e}"))
        })
        .collect()
}

fn part1(input: &[u16], nails: u16) -> u32 {
    input
        .iter()
        .tuple_windows()
        .filter(|(a, b)| *a.max(b) - *a.min(b) == nails / 2)
        .count() as u32
}

fn part2(input: &[u16]) -> u64 {
    let mut result = 0;
    let lines = input
        .windows(2)
        .map(|window| window.to_vec())
        .collect::<Vec<Vec<u16>>>();
    for i in 0..lines.len() {
        let smaller_point = lines[i].iter().min().unwrap();
        let larger_point = lines[i].iter().max().unwrap();
        result += lines
            .iter()
            .skip(i + 1)
            .filter(|window| {
                ((window[0] < *larger_point && window[0] > *smaller_point)
                    && (window[1] > *larger_point || window[1] < *smaller_point))
                    || ((window[1] < *larger_point && window[1] > *smaller_point)
                        && (window[0] > *larger_point || window[0] < *smaller_point))
            })
            .count() as u64;
    }
    result
}
