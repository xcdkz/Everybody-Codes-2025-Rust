use std::fs;

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => println!("Test 1: {}", part1(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok(input) => println!("Part 1: {}", part1(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/test2.txt") {
        Ok(input) => println!("Test 2: {}", part2(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input2.txt") {
        Ok(input) => println!("Part 2: {}", part2(&input)),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/test3.txt") {
        Ok(input) => println!("Test 3: {}", part3(&input, 10, 1).unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
    match extract_input_from_file("input/input3.txt") {
        Ok(input) => println!("Part 3: {}", part3(&input, 1000, 1000).unwrap()),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn part3(input: &[u8], max_distance: usize, repeat: usize) -> Result<usize, String> {
    if repeat == 0 {
        return Err("Repeat count must be greater than 0".to_string());
    }
    let first_novices_input_repeat = input[input.len() - max_distance..]
        .iter()
        .chain(input[..max_distance * 2].iter())
        .cloned()
        .collect::<Vec<u8>>();
    let last_novices_input_repeat = input[input.len() - max_distance * 2..]
        .iter()
        .chain(input[..max_distance].iter())
        .cloned()
        .collect::<Vec<u8>>();
    let first_novices_knights_repeat = first_novices_input_repeat
        .iter()
        .enumerate()
        .skip(max_distance)
        .take(max_distance + 1)
        .fold(0, |acc, (index, &byte)| {
            if byte == b'a' || byte == b'b' || byte == b'c' {
                acc + valid_knights(&first_novices_input_repeat, index, max_distance).unwrap()
            } else {
                acc
            }
        });
    let last_novices_knights_repeat = last_novices_input_repeat
        .iter()
        .enumerate()
        .skip(max_distance)
        .take(max_distance + 1)
        .fold(0, |acc, (index, &byte)| {
            if byte == b'a' || byte == b'b' || byte == b'c' {
                acc + valid_knights(&last_novices_input_repeat, index, max_distance).unwrap()
            } else {
                acc
            }
        });

    Ok(
        (repeat - 1) * (first_novices_knights_repeat + last_novices_knights_repeat)
            + input
                .iter()
                .enumerate()
                .take(max_distance)
                .fold(0, |acc, (index, &byte)| {
                    if byte == b'a' || byte == b'b' || byte == b'c' {
                        acc + valid_knights(&input, index, max_distance).unwrap()
                    } else {
                        acc
                    }
                })
            + input
                .iter()
                .enumerate()
                .skip(input.len() - max_distance - 1)
                .fold(0, |acc, (index, &byte)| {
                    if byte == b'a' || byte == b'b' || byte == b'c' {
                        acc + valid_knights(&input, index, max_distance).unwrap()
                    } else {
                        acc
                    }
                })
            + input
                .iter()
                .enumerate()
                .skip(max_distance)
                .take(input.len() - max_distance * 2 - 1)
                .fold(0, |acc, (index, &byte)| {
                    if byte == b'a' || byte == b'b' || byte == b'c' {
                        acc + valid_knights(&input, index, max_distance).unwrap()
                    } else {
                        acc
                    }
                })
                * repeat,
    )
}

fn valid_knights(input: &[u8], index: usize, max_distance: usize) -> Result<usize, String> {
    let knight_type = match input[index] {
        b'a' => b'A',
        b'b' => b'B',
        b'c' => b'C',
        _ => return Err("Invalid knight type".to_string()),
    };
    let start_pos = index.saturating_sub(max_distance);
    let end_pos = (index + max_distance).min(input.len());
    Ok(input
        .iter()
        .skip(start_pos)
        .take(end_pos - start_pos + 1)
        .filter(|&byte| *byte == knight_type)
        .count())
}

fn part1(input: &[u8]) -> usize {
    let mut a_knights = 0;
    input.iter().fold(0, |acc, &byte| match byte {
        b'A' => {
            a_knights += 1;
            acc
        }
        b'a' => acc + a_knights,
        _ => acc,
    })
}

fn part2(input: &[u8]) -> usize {
    let mut a_knights = 0;
    let mut b_knights = 0;
    let mut c_knights = 0;
    input.iter().fold(0, |acc, &byte| match byte {
        b'A' => {
            a_knights += 1;
            acc
        }
        b'a' => acc + a_knights,
        b'B' => {
            b_knights += 1;
            acc
        }
        b'b' => acc + b_knights,
        b'C' => {
            c_knights += 1;
            acc
        }
        b'c' => acc + c_knights,
        _ => acc,
    })
}

fn extract_input_from_file(file_path: &str) -> Result<Vec<u8>, String> {
    Ok(fs::read_to_string(file_path)
        .map_err(|err| format!("Error reading file: {}", err))?
        .trim()
        .as_bytes()
        .to_vec())
}
