use std::collections::{HashMap, HashSet};

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => println!("Test 1: {:?}", part1(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok(input) => println!("Input 1: {:?}", part1(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn part1(input: &(Vec<String>, HashMap<u8, HashSet<u8>>)) -> Option<String> {
    for name in &input.0 {
        let mut valid_name = true;
        for name_letter_index in 0..name.len() - 1 {
            let current_letter = name.as_bytes()[name_letter_index];
            let next_letter = name.as_bytes()[name_letter_index + 1];
            if !input.1[&current_letter].contains(&next_letter) {
                valid_name = false;
                break;
            }
        }
        if valid_name {
            return Some(name.clone());
        }
    }
    None
}

fn extract_input_from_file(
    file_path: &str,
) -> Result<(Vec<String>, HashMap<u8, HashSet<u8>>), String> {
    let contents = std::fs::read_to_string(file_path).map_err(|e| e.to_string())?;

    let names = contents
        .trim()
        .lines()
        .next()
        .ok_or("No names line found".to_string())?
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    let letter_rules = contents
        .trim()
        .lines()
        .filter(|line| line.contains('>'))
        .map(|line| {
            let (left, right) = line
                .split_once(" > ")
                .ok_or(format!("Invalid rule format: {line}"))?;

            let key = left
                .trim()
                .bytes()
                .next()
                .ok_or(format!("No key byte in: {left}"))?;

            let values = right
                .split(',')
                .map(|s| {
                    s.trim()
                        .bytes()
                        .next()
                        .ok_or_else(|| format!("Invalid byte in values: '{s}' (rule: {line})"))
                })
                .collect::<Result<HashSet<u8>, String>>()?;

            Ok((key, values))
        })
        .collect::<Result<HashMap<u8, HashSet<u8>>, String>>()?;

    Ok((names, letter_rules))
}
