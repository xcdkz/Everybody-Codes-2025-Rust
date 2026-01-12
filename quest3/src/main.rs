use itertools::Itertools;
use std::fs;

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(v) => println!("Test1: {}", part1(&v)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok(v) => println!("Part1: {}", part1(&v)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/test2.txt") {
        Ok(v) => println!("Test2: {}", part2(&v)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/input2.txt") {
        Ok(v) => println!("Part2: {}", part2(&v)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/test3.txt") {
        Ok(v) => println!("Test3: {}", part3(&v)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/input3.txt") {
        Ok(v) => println!("Part3: {}", part3(&v)),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn extract_input_from_file(file_path: &str) -> Result<Vec<u32>, String> {
    let unparsed_input = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read the input file \"{}\": {}", file_path, e))?;

    unparsed_input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<u32>().map_err(|e| {
                format!(
                    "Couldn't parse \"{}\" to u32 from the input file: {} - {}",
                    s, file_path, e
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

fn part1(input: &[u32]) -> u32 {
    input.iter().unique().sum()
}

fn part2(input: &[u32]) -> u32 {
    input.iter().unique().sorted().take(20).sum()
}

fn part3(input: &[u32]) -> u32 {
    input
        .iter()
        .duplicates()
        .map(|duplicate_crate| input.iter().filter(|v| *v == duplicate_crate).count())
        .max()
        .expect("Empty input") as u32
}
