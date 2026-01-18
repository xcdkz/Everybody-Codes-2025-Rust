use std::fs;

use itertools::Itertools;

struct Line {
    a: u16,
    b: u16,
}

impl Line {
    fn new(a: u16, b: u16) -> Self {
        Line {
            a: a.min(b),
            b: a.max(b),
        }
    }
    fn cuts(&self, line: &Line) -> bool {
        ((self.a < line.b && self.a > line.a) && (self.b > line.b))
            || ((self.b > line.a && self.b < line.b) && (self.a < line.a))
    }
}

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
    match extract_input_from_file("input/test3.txt") {
        Ok(input) => println!("Test 3: {}", part3(&input, 8)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input3.txt") {
        Ok(input) => println!("Input 3: {}", part3(&input, 256)),
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

fn part3(input: &[u16], nails: u16) -> u32 {
    let lines = input
        .windows(2)
        .map(|window| Line::new(window[0], window[1]))
        .collect::<Vec<Line>>();
    let mut result = 0;
    for i in 0..nails {
        for j in i + 1..=nails {
            result = result.max(
                lines
                    .iter()
                    .filter(|line| Line { a: i, b: j }.cuts(line) || (line.a == i && line.b == j))
                    .count() as u32,
            );
        }
    }
    result
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
        .map(|window| Line::new(window[0], window[1]))
        .collect::<Vec<Line>>();
    for i in 0..lines.len() {
        result += lines
            .iter()
            .skip(i + 1)
            .filter(|line| line.cuts(&lines[i]))
            .count() as u64;
    }
    result
}
