use std::fs;

struct FishboneNode {
    value: u32,
    lhs: Option<u32>,
    rhs: Option<u32>,
}

fn main() {
    match extract_input_from_file_part1("input/test1.txt") {
        Ok(input) => println!("Test 1: {}", part1(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file_part1("input/input1.txt") {
        Ok(input) => println!("Part 1: {}", part1(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file_part2("input/test2.txt") {
        Ok(input) => println!("Test 2: {:?}", part2(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file_part2("input/input2.txt") {
        Ok(input) => println!("Part 2: {}", part2(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file_part3("input/test3.txt") {
        Ok(input) => println!("Test 3: {:?}", part3(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file_part3("input/input3.txt") {
        Ok(input) => println!("Part 3: {}", part3(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn part1(input: &[u32]) -> String {
    let mut fishbone = vec![FishboneNode {
        value: input[0],
        lhs: None,
        rhs: None,
    }];
    for num in input.iter().skip(1) {
        let mut num_placed = false;
        for node in &mut fishbone {
            if node.lhs.is_none() && *num < node.value {
                node.lhs = Some(*num);
                num_placed = true;
                break;
            } else if node.rhs.is_none() && *num > node.value {
                node.rhs = Some(*num);
                num_placed = true;
                break;
            }
        }
        if !num_placed {
            fishbone.push(FishboneNode {
                value: *num,
                lhs: None,
                rhs: None,
            });
        }
    }
    fishbone[0].value.to_string()
        + &fishbone
            .iter()
            .skip(1)
            .map(|node| node.value.to_string())
            .fold(String::new(), |acc, s| acc + &s)
}

fn part2(input: &[Vec<u32>]) -> u64 {
    let mut sword_values = Vec::new();
    for sword in input {
        let mut fishbone = vec![FishboneNode {
            value: sword[0],
            lhs: None,
            rhs: None,
        }];
        for num in sword.iter().skip(1) {
            let mut num_placed = false;
            for node in &mut fishbone {
                if node.lhs.is_none() && *num < node.value {
                    node.lhs = Some(*num);
                    num_placed = true;
                    break;
                } else if node.rhs.is_none() && *num > node.value {
                    node.rhs = Some(*num);
                    num_placed = true;
                    break;
                }
            }
            if !num_placed {
                fishbone.push(FishboneNode {
                    value: *num,
                    lhs: None,
                    rhs: None,
                });
            }
        }
        sword_values.push(
            (fishbone[0].value.to_string()
                + &fishbone
                    .iter()
                    .skip(1)
                    .map(|node| node.value.to_string())
                    .fold(String::new(), |acc, s| acc + &s))
                .parse::<u64>()
                .unwrap(),
        );
    }
    sword_values.iter().max().unwrap() - sword_values.iter().min().unwrap()
}

fn part3(input: &[Vec<Vec<u32>>]) -> u64 {
    todo!()
}

fn extract_input_from_file_part1(file_path: &str) -> Result<Vec<u32>, String> {
    fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?
        .trim()
        .split_once(':')
        .ok_or("Invalid input format")?
        .1
        .split(',')
        .map(|num| {
            num.parse()
                .map_err(|e| format!("Failed to parse number: {}", e))
        })
        .collect()
}

fn extract_input_from_file_part2(file_path: &str) -> Result<Vec<Vec<u32>>, String> {
    fs::read_to_string(file_path)
        .map_err(|e| e.to_string())?
        .trim()
        .lines()
        .map(|line| {
            line.split_once(':')
                .ok_or(format!("Invalid input format for line: {}", line))?
                .1
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.parse::<u32>()
                        .map_err(|e| format!("parse error: {} → {}", s, e))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect()
}

fn extract_input_from_file_part3(file_path: &str) -> Result<Vec<Vec<Vec<u32>>>, String> {
    todo!()
}
