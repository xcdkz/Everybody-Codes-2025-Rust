#[derive(Debug, Clone)]
struct DeoxyribonucleicAcid {
    id: u32,
    sequence: Vec<u8>,
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
}

fn part1(input: &[DeoxyribonucleicAcid]) -> Result<u32, String> {
    if input.len() != 3 {
        return Err("Input must contain exactly three DNA sequences".to_string());
    }
    let dnas: [DeoxyribonucleicAcid; 3] = input
        .to_vec()
        .try_into()
        .map_err(|_| "Input must contain exactly three DNA sequences")?;

    let child_id = part1_find_child_id(&dnas)?;
    let child = dnas
        .iter()
        .find(|element| element.id == child_id)
        .ok_or("Child not found")?;

    let parent_a = dnas
        .iter()
        .filter(|element| element.id != child.id)
        .min_by_key(|element| element.id)
        .ok_or("No parent found")?;
    let parent_b = dnas
        .iter()
        .filter(|element| element.id != child.id)
        .max_by_key(|element| element.id)
        .ok_or("No parent found")?;
    Ok((parent_a
        .sequence
        .iter()
        .enumerate()
        .filter(|(i, parent_symbol)| **parent_symbol == child.sequence[*i])
        .count()
        * parent_b
            .sequence
            .iter()
            .enumerate()
            .filter(|(i, parent_symbol)| **parent_symbol == child.sequence[*i])
            .count()) as u32)
}

fn part1_find_child_id(dnas: &[DeoxyribonucleicAcid; 3]) -> Result<u32, String> {
    let [a, b, c] = dnas;
    let dna_sequence_len = a.sequence.len();
    let mut possible_childs = vec![&a, &b, &c];
    for i in 0..dna_sequence_len {
        if possible_childs.len() == 1 {
            break;
        }
        if possible_childs.iter().any(|element| element.id == a.id)
            && !(a.sequence[i] == b.sequence[i] || a.sequence[i] == c.sequence[i])
        {
            possible_childs.retain(|element| element.id != a.id);
        }
        if possible_childs.iter().any(|element| element.id == b.id)
            && !(b.sequence[i] == a.sequence[i] || b.sequence[i] == c.sequence[i])
        {
            possible_childs.retain(|element| element.id != b.id);
        }
        if possible_childs.iter().any(|element| element.id == c.id)
            && !(c.sequence[i] == b.sequence[i] || c.sequence[i] == a.sequence[i])
        {
            possible_childs.retain(|element| element.id != c.id);
        }
    }
    let child = possible_childs.first().ok_or("No child found")?;
    Ok(child.id)
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
