use std::collections::HashSet;

#[derive(Debug, Clone)]
struct DeoxyribonucleicAcid {
    id: u32,
    sequence: Vec<u8>,
}

impl DeoxyribonucleicAcid {
    fn find_parents(
        &self,
        dragonducks: &[DeoxyribonucleicAcid],
    ) -> Option<[DeoxyribonucleicAcid; 2]> {
        for i in 0..dragonducks.len() {
            if dragonducks[i].id == self.id {
                continue;
            }
            for j in i + 1..dragonducks.len() {
                if dragonducks[j].id == self.id {
                    continue;
                }
                let parents = [&dragonducks[i], &dragonducks[j]];
                if self.are_valid_parents(&parents) {
                    return Some([dragonducks[i].clone(), dragonducks[j].clone()]);
                }
            }
        }
        None
    }

    fn are_valid_parents(&self, parents: &[&DeoxyribonucleicAcid; 2]) -> bool {
        self.sequence.iter().enumerate().all(|(i, element)| {
            *element == parents[0].sequence[i] || *element == parents[1].sequence[i]
        })
    }

    fn count_similarity_degree(&self, parent: &DeoxyribonucleicAcid) -> usize {
        self.sequence
            .iter()
            .enumerate()
            .filter(|(i, element)| **element == parent.sequence[*i])
            .count()
    }
}

#[derive(Clone)]
struct Dragonduck {
    dna: DeoxyribonucleicAcid,
    parents: Option<[DeoxyribonucleicAcid; 2]>,
}

impl Dragonduck {
    fn find_children(&self, dragonducks: &[Dragonduck]) -> Vec<Dragonduck> {
        dragonducks
            .iter()
            .filter(|dragonduck| {
                dragonduck
                    .parents
                    .as_ref()
                    .is_some_and(|parents| parents.iter().any(|parent| parent.id == self.dna.id))
            })
            .cloned()
            .collect()
    }
    // fn build_family(&self, dragonducks: &[Dragonduck]) -> Result<HashSet<Dragonduck>, String> {
    //     let mut result = HashSet::new();
    //     if let Some(parents) = self.parents {
    //         if !result.contains(&parents[0]) {
    //             result.union(
    //                 dragonducks
    //                     .iter()
    //                     .find(|dragonduck| dragonduck.dna.id == parents[0].id)
    //                     .ok_or("Couldn't find dragonduck")?
    //                     .build_family(dragonducks)
    //                     .map_err(|e| format!("Couldn't build family", e))?,
    //             )
    //         }
    //     }
    // }
}

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => println!("Test1: {:?}", part1(&input)),
        Err(err) => println!("Error: {}", err),
    }
    match extract_input_from_file("input/input1.txt") {
        Ok(input) => println!("Input1: {:?}", part1(&input)),
        Err(err) => println!("Error: {}", err),
    }
    match extract_input_from_file("input/test2.txt") {
        Ok(input) => println!("Test2: {}", part2(&input)),
        Err(err) => println!("Error: {}", err),
    }
    match extract_input_from_file("input/input2.txt") {
        Ok(input) => println!("Input2: {}", part2(&input)),
        Err(err) => println!("Error: {}", err),
    }
    match extract_input_from_file("input/test3_1.txt") {
        Ok(input) => println!("Test3_1: {}", part3(&input)),
        Err(err) => println!("Error: {}", err),
    }
    match extract_input_from_file("input/test3_2.txt") {
        Ok(input) => println!("Test3_2: {}", part3(&input)),
        Err(err) => println!("Error: {}", err),
    }
    match extract_input_from_file("input/input3.txt") {
        Ok(input) => println!("Input3: {}", part3(&input)),
        Err(err) => println!("Error: {}", err),
    }
}

fn part3(input: &[DeoxyribonucleicAcid]) -> usize {
    let dragonducks: Vec<Dragonduck> = input
        .iter()
        .map(|dragonduck| Dragonduck {
            dna: dragonduck.clone(),
            parents: dragonduck.find_parents(input),
        })
        .collect();
    let oldest_dragonducks: Vec<&Dragonduck> = dragonducks
        .iter()
        .filter(|dragonduck| dragonduck.parents.is_none())
        .collect();

    todo!()
}

fn part2(input: &[DeoxyribonucleicAcid]) -> usize {
    input.iter().fold(0, |acc, dragonduck| {
        if let Some(parents) = dragonduck.find_parents(input) {
            dragonduck.count_similarity_degree(&parents[0])
                * dragonduck.count_similarity_degree(&parents[1])
                + acc
        } else {
            acc
        }
    })
}

fn part1(input: &[DeoxyribonucleicAcid]) -> Result<usize, String> {
    let child = input
        .iter()
        .find(|deoxyribonucleic_acid| deoxyribonucleic_acid.find_parents(input).is_some())
        .ok_or("Couldn't find a valid child")?;
    let parents = child.find_parents(input).ok_or("Couldn't find parents")?;
    Ok(child.count_similarity_degree(&parents[0]) * child.count_similarity_degree(&parents[1]))
}

fn extract_input_from_file(file_path: &str) -> Result<Vec<DeoxyribonucleicAcid>, String> {
    std::fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?
        .trim()
        .lines()
        .map(|line| {
            let parts = line
                .split_once(':')
                .ok_or(format!("Couldn't split line with ':': {}", line))?;
            let id = parts.0.parse().map_err(|e| format!("Invalid ID: {}", e))?;
            let sequence = parts.1.bytes().collect();
            Ok(DeoxyribonucleicAcid { id, sequence })
        })
        .collect()
}
