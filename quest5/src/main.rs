use std::fs;

struct Fishbone {
    id: u32,
    nodes: Vec<FishboneNode>,
}

struct SimplifiedFishbone {
    id: u32,
    nodes: Vec<u32>,
}

struct FishboneNode {
    value: u32,
    lhs: Option<u32>,
    rhs: Option<u32>,
}

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => println!("Test 1: {}", part1(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok(input) => println!("Part 1: {}", part1(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/test2.txt") {
        Ok(input) => println!("Test 2: {:?}", part2(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/input2.txt") {
        Ok(input) => println!("Part 2: {}", part2(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    // match extract_input_from_file("input/test3.txt") {
    //     Ok(input) => println!("Test 3: {:?}", part3(&input)),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
    // match extract_input_from_file("input/input3.txt") {
    //     Ok(input) => println!("Part 3: {}", part3(&input)),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
}

fn part1(input: &[SimplifiedFishbone]) -> String {
    let mut fishbone = vec![FishboneNode {
        value: input[0].nodes[0],
        lhs: None,
        rhs: None,
    }];
    for num in input[0].nodes.iter().skip(1) {
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

fn part2(input: &[SimplifiedFishbone]) -> u64 {
    let mut sword_values = Vec::new();
    for sword in input {
        let mut fishbone = vec![FishboneNode {
            value: sword.nodes[0],
            lhs: None,
            rhs: None,
        }];
        for num in sword.nodes.iter().skip(1) {
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

fn part3(input: &[SimplifiedFishbone]) -> u64 {
    todo!()
}

fn extract_input_from_file(file_path: &str) -> Result<Vec<SimplifiedFishbone>, String> {
    fs::read_to_string(file_path)
        .map_err(|e| e.to_string())?
        .trim()
        .lines()
        .map(|line| {
            let (id_str, nodes_str) = line.split_once(':').ok_or("missing ':'")?;

            let id = id_str
                .trim()
                .parse::<u32>()
                .map_err(|_| format!("invalid id: {:?}", id_str))?;

            let nodes = nodes_str
                .split(',')
                .map(|s| {
                    s.parse::<u32>()
                        .map_err(|e| format!("invalid number {:?}: {}", s, e))
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(SimplifiedFishbone { id, nodes })
        })
        .collect()
}
