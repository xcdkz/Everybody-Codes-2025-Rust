use core::{clone::Clone, iter::Iterator};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct InstructionSet {
    names: Vec<String>,
    rules: HashMap<u8, HashSet<u8>>,
}

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => println!("Test 1: {:?}", part1(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok(input) => println!("Input 1: {:?}", part1(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/test2.txt") {
        Ok(input) => println!("Test 2: {:?}", part2(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input2.txt") {
        Ok(input) => println!("Input 2: {:?}", part2(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/test3.txt") {
        Ok(input) => println!("Test 3: {:?}", part3(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/test4.txt") {
        Ok(input) => println!("Test 4: {:?}", part3(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input3.txt") {
        Ok(input) => println!("Input 3: {:?}", part3(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn part1(input: &InstructionSet) -> Option<String> {
    input
        .names
        .iter()
        .find(|name| is_name_valid(name, &input.rules))
        .cloned()
}

fn part2(input: &InstructionSet) -> u32 {
    input
        .names
        .iter()
        .enumerate()
        .fold(0, |acc, (index, name)| {
            if is_name_valid(name, &input.rules) {
                acc + (index + 1) as u32
            } else {
                acc
            }
        })
}

// 7 to 11 letters
// only valid names
fn part3(input: &InstructionSet) -> u64 {
    let valid_names = input
        .names
        .iter()
        .filter(|name| is_name_valid(name, &input.rules))
        .collect::<Vec<&String>>();
    let filtered_valid_names = filter_extended_names(&valid_names);
    filtered_valid_names.iter().fold(0, |acc, name| {
        acc + count_possible_unique_names(7, 11, name.as_bytes(), &input.rules)
    })
}

fn filter_extended_names(names: &Vec<&String>) -> Vec<String> {
    let mut names_mut: Vec<String> = names
        .iter()
        .map(|name| name.to_string())
        .collect::<Vec<String>>();
    names_mut.sort_by_key(|s| s.len());
    let mut result = vec![names_mut[0].clone()];
    for i in 0..names_mut.len() - 1 {
        let cur_name = &names_mut[names_mut.len() - 1 - i];
        if !names_mut[0..names_mut.len() - 2 - i]
            .iter()
            .any(|name| cur_name.starts_with(name))
        {
            result.push(cur_name.clone());
        }
    }
    result
}

fn count_possible_unique_names(
    min_chars: u8,
    max_chars: u8,
    base: &[u8],
    rules: &HashMap<u8, HashSet<u8>>,
) -> u64 {
    internal_count_possible_unique_names(
        max_chars,
        *base.iter().last().unwrap(),
        base.len() as u8,
        rules,
    ) - internal_count_possible_unique_names(
        min_chars - 1,
        *base.iter().last().unwrap(),
        base.len() as u8,
        rules,
    )
}

fn internal_count_possible_unique_names(
    max_chars: u8,
    last_byte: u8,
    chars_len: u8,
    rules: &HashMap<u8, HashSet<u8>>,
) -> u64 {
    if chars_len == max_chars {
        1
    } else if let Some(bytes) = rules.get(&last_byte) {
        bytes.iter().fold(1, |acc, byte| {
            acc + internal_count_possible_unique_names(max_chars, *byte, chars_len + 1, rules)
        })
    } else {
        1
    }
}

fn is_name_valid(name: &str, rules: &HashMap<u8, HashSet<u8>>) -> bool {
    for name_letter_index in 0..name.len() - 1 {
        let current_letter = name.as_bytes()[name_letter_index];
        let next_letter = name.as_bytes()[name_letter_index + 1];
        if !rules[&current_letter].contains(&next_letter) {
            return false;
        }
    }
    true
}

fn extract_input_from_file(file_path: &str) -> Result<InstructionSet, String> {
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

    Ok(InstructionSet {
        names,
        rules: letter_rules,
    })
}
