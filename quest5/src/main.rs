use std::fs;

#[derive(Debug)]
struct Fishbone {
    id: u32,
    nodes: Vec<FishboneNode>,
}

struct SimplifiedFishbone {
    id: u32,
    nodes: Vec<u32>,
}

#[derive(Debug)]
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
    match extract_input_from_file("input/test3.txt") {
        Ok(input) => println!("Test 3: {:?}", part3(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
    match extract_input_from_file("input/input3.txt") {
        Ok(input) => println!("Part 3: {}", part3(&input)),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn part1(input: &[SimplifiedFishbone]) -> String {
    let mut fishbone = Fishbone {
        id: input[0].id,
        nodes: vec![FishboneNode {
            value: input[0].nodes[0],
            lhs: None,
            rhs: None,
        }],
    };
    for num in input[0].nodes.iter().skip(1) {
        let mut num_placed = false;
        for node in &mut fishbone.nodes {
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
            fishbone.nodes.push(FishboneNode {
                value: *num,
                lhs: None,
                rhs: None,
            });
        }
    }
    fishbone.nodes[0].value.to_string()
        + &fishbone
            .nodes
            .iter()
            .skip(1)
            .map(|node| node.value.to_string())
            .fold(String::new(), |acc, s| acc + &s)
}

fn part2(input: &[SimplifiedFishbone]) -> u64 {
    let mut sword_values = Vec::new();
    for sword in input {
        let mut fishbone = Fishbone {
            id: sword.id,
            nodes: vec![FishboneNode {
                value: sword.nodes[0],
                lhs: None,
                rhs: None,
            }],
        };
        for num in sword.nodes.iter().skip(1) {
            let mut num_placed = false;
            for node in &mut fishbone.nodes {
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
                fishbone.nodes.push(FishboneNode {
                    value: *num,
                    lhs: None,
                    rhs: None,
                });
            }
        }
        sword_values.push(
            (fishbone.nodes[0].value.to_string()
                + &fishbone
                    .nodes
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
    let mut sword_fishbones = Vec::new();
    for sword in input {
        let mut fishbone = Fishbone {
            id: sword.id,
            nodes: vec![FishboneNode {
                value: sword.nodes[0],
                lhs: None,
                rhs: None,
            }],
        };
        for num in sword.nodes.iter().skip(1) {
            let mut num_placed = false;
            for node in &mut fishbone.nodes {
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
                fishbone.nodes.push(FishboneNode {
                    value: *num,
                    lhs: None,
                    rhs: None,
                });
            }
        }
        sword_fishbones.push(fishbone);
    }
    // for fishbone in &sword_fishbones {
    //     println!("{:?}", fishbone);
    // }
    sword_fishbones.sort_by(|a, b| {
        let a_quality = a
            .nodes
            .iter()
            .map(|node| node.value.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        let b_quality = b
            .nodes
            .iter()
            .map(|node| node.value.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        if a_quality != b_quality {
            b_quality.cmp(&a_quality)
        } else {
            for i in 0..a.nodes.len() {
                let a_level_lhs_value = if a.nodes[i].lhs.is_none() {
                    String::from("")
                } else {
                    a.nodes[i].lhs.unwrap().to_string()
                };
                let a_level_rhs_value = if a.nodes[i].rhs.is_none() {
                    String::from("")
                } else {
                    a.nodes[i].rhs.unwrap().to_string()
                };
                let b_level_lhs_value = if b.nodes[i].lhs.is_none() {
                    String::from("")
                } else {
                    b.nodes[i].lhs.unwrap().to_string()
                };
                let b_level_rhs_value = if b.nodes[i].rhs.is_none() {
                    String::from("")
                } else {
                    b.nodes[i].rhs.unwrap().to_string()
                };
                let a_level_value =
                    (a_level_lhs_value + &a.nodes[i].value.to_string() + &a_level_rhs_value)
                        .parse::<u64>()
                        .unwrap();
                let b_level_value =
                    (b_level_lhs_value + &b.nodes[i].value.to_string() + &b_level_rhs_value)
                        .parse::<u64>()
                        .unwrap();
                if a_level_value != b_level_value {
                    return b_level_value.cmp(&a_level_value);
                }
            }
            b.id.cmp(&a.id)
        }
    });
    sword_fishbones
        .iter()
        .enumerate()
        .fold(0, |acc, (i, fishbone)| {
            acc + (((i as u64) + 1) * (fishbone.id as u64))
        })
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
