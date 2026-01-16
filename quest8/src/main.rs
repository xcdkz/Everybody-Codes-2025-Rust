use std::fs;

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => println!("Test 1: {}", part1(&input, 8)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok(input) => println!("Input 1: {}", part1(&input, 32)),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn extract_input_from_file(file_path: &str) -> Result<Vec<u8>, String> {
    fs::read_to_string(file_path)
        .map_err(|e| e.to_string())?
        .trim()
        .split(',')
        .map(|n| {
            n.parse::<u8>()
                .map_err(|e| format!("Couldn't parse number: {n} - {e}"))
        })
        .collect()
}

fn part1(input: &[u8], nails: u8) -> u32 {
    input
        .windows(2)
        .filter(|window| window[0].max(window[1]) - window[0].min(window[1]) == nails / 2)
        .count() as u32
}
