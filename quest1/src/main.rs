use std::fs;

#[derive(Debug)]
struct Instruction {
    direction: u8,
    distance: u32,
}

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok((names, instructions)) => println!("Test 1: {}", part1(&names, &instructions)),
        Err(e) => println!("Error: {}", e),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok((names, instructions)) => println!("Part 1: {}", part1(&names, &instructions)),
        Err(e) => println!("Error: {}", e),
    }
    match extract_input_from_file("input/test2.txt") {
        Ok((names, instructions)) => println!("Test 2: {}", part2(&names, &instructions)),
        Err(e) => println!("Error: {}", e),
    }
    match extract_input_from_file("input/input2.txt") {
        Ok((names, instructions)) => println!("Part 2: {}", part2(&names, &instructions)),
        Err(e) => println!("Error: {}", e),
    }
    match extract_input_from_file("input/test3.txt") {
        Ok((names, instructions)) => println!("Test 3: {}", part3(&names, &instructions)),
        Err(e) => println!("Error: {}", e),
    }
    match extract_input_from_file("input/input3.txt") {
        Ok((names, instructions)) => println!("Part 3: {}", part3(&names, &instructions)),
        Err(e) => println!("Error: {}", e),
    }
}

fn part1(names: &[String], instructions: &[Instruction]) -> String {
    let mut current_name_index: u32 = 0;
    instructions
        .iter()
        .for_each(|instruction| match instruction.direction {
            b'L' => current_name_index = current_name_index.saturating_sub(instruction.distance),
            b'R' => {
                current_name_index =
                    ((names.len() - 1) as u32).min(current_name_index + instruction.distance)
            }
            _ => panic!("Invalid direction"),
        });

    names[current_name_index as usize].clone()
}

fn part2(names: &[String], instructions: &[Instruction]) -> String {
    let mut current_name_index: u32 = 0;
    instructions
        .iter()
        .for_each(|instruction| match instruction.direction {
            b'L' => {
                current_name_index = if instruction.distance > current_name_index {
                    names.len() as u32
                        - 1
                        - (instruction.distance - (current_name_index + 1)) % (names.len() as u32)
                } else {
                    current_name_index - instruction.distance
                };
            }
            b'R' => {
                current_name_index =
                    (instruction.distance + current_name_index) % (names.len() as u32)
            }
            _ => panic!("Invalid direction"),
        });

    names[current_name_index as usize].clone()
}

fn part3(names: &[String], instructions: &[Instruction]) -> String {
    let max_index = (names.len() - 1) as u32;
    let mut names_vec_cloned = names.to_vec();
    let mut swap_position_with;

    for instruction in instructions {
        match instruction.direction {
            b'L' => {
                swap_position_with = if instruction.distance.is_multiple_of(max_index + 1) {
                    0
                } else {
                    (max_index + 1) - (instruction.distance % (max_index + 1))
                }
            }
            b'R' => swap_position_with = (instruction.distance) % (max_index + 1),
            _ => panic!("Invalid direction"),
        }
        let tmp_container = names_vec_cloned[swap_position_with as usize].clone();
        names_vec_cloned[swap_position_with as usize] = names_vec_cloned[0].clone();
        names_vec_cloned[0] = tmp_container;
    }

    names_vec_cloned[0].clone()
}

fn extract_input_from_file(file_path: &str) -> Result<(Vec<String>, Vec<Instruction>), String> {
    let unparsed_input = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read the input file {}: {}", file_path, e))?;

    let names = unparsed_input
        .trim()
        .lines()
        .next()
        .ok_or_else(|| format!("Failed to parse names from file {}", file_path))?
        .split(',')
        .map(|name| name.to_string())
        .collect::<Vec<String>>();

    let instructions = unparsed_input
        .trim()
        .lines()
        .last()
        .ok_or_else(|| format!("Failed to parse instructions from file {}", file_path))?
        .split(',')
        .map(|instruction| Instruction {
            direction: instruction.as_bytes()[0],
            distance: instruction[1..].parse().unwrap(),
        })
        .collect::<Vec<Instruction>>();

    Ok((names, instructions))
}
