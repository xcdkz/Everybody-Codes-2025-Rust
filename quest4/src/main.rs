use std::{fs, ops::Mul};

use itertools::Itertools;

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => println!("Test 1: {}", part1(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok(input) => println!("Part 1: {}", part1(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => println!("Test 2: {}", part2(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/input2.txt") {
        Ok(input) => println!("Part 2: {}", part2(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file_part3("input/test3.txt") {
        Ok(input) => println!("Test 3: {}", part3(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file_part3("input/input3.txt") {
        Ok(input) => println!("Input 3: {}", part3(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn extract_input_from_file(file_path: &str) -> Result<Vec<u32>, String> {
    fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))
        .and_then(|content| {
            content
                .lines()
                .map(|line| line.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()
                .map_err(|e| format!("Failed to parse line: {}", e))
        })
}

fn extract_input_from_file_part3(file_path: &str) -> Result<Vec<(u32, u32)>, String> {
    let unparsed_input =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    let parsed_input = format!("0|{}|0", unparsed_input.trim());

    parsed_input
        .lines()
        .map(|line| {
            let parts: (&str, &str) = line
                .split_once('|')
                .ok_or_else(|| format!("Invalid format: {}", line))?;
            let num1: u32 = parts
                .0
                .parse::<u32>()
                .map_err(|e| format!("Failed to parse number 1: {}", e))?;
            let num2: u32 = parts
                .1
                .parse::<u32>()
                .map_err(|e| format!("Failed to parse number 2: {}", e))?;
            Ok((num1, num2))
        })
        .collect::<Result<Vec<(u32, u32)>, _>>()
}

fn part1(input: &[u32]) -> u32 {
    input
        .iter()
        .map(|x| *x as f64)
        .tuple_windows()
        .fold(1.0, |acc, (a, b)| acc * (a / b))
        .mul(2025.0)
        .floor() as u32
}

fn part2(input: &[u32]) -> u64 {
    (10000000000000.0
        / input
            .iter()
            .map(|x| *x as f64)
            .tuple_windows()
            .fold(1.0, |acc, (a, b)| acc * (a / b)))
    .ceil() as u64
}

fn part3(input: &[(u32, u32)]) -> u64 {
    input
        .iter()
        .map(|(a, b)| (*a as f64, *b as f64))
        .tuple_windows()
        .fold(1.0, |acc, ((_, b1), (a2, _))| acc * (b1 / a2))
        .mul(100.0)
        .floor() as u64
}
