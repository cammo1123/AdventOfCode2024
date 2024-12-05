use std::{collections::HashMap, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";
    let contents = fs::read_to_string(file_path)?;
    let (rules, updates) = parse_input(&contents);

    let valid_total: u32 = updates
        .clone()
        .into_iter()
        .filter_map(|update| { 
            if is_valid_update(&update, &rules, 0) {
                return update.get(update.len() / 2).cloned()
            }             
            None
        })
        .sum();

    let invalid_total: u32 = updates
        .clone()
        .into_iter()
        .filter_map(|mut update| {
            if !is_valid_update(&update, &rules, 0) { 
                make_valid_update(&mut update, &rules, 0); 
                return update.get(update.len() / 2).cloned();
            }
            None 
        })
        .sum();
    
    println!("Part One: {}", valid_total);
    println!("Part Two: {}", invalid_total);

    Ok(())
}

fn parse_input(contents: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let parts: Vec<&str> = contents.split("\r\n\r\n").collect();

    let rules: HashMap<u32, Vec<u32>> = parts[0]
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line.split('|').map(|num| num.parse().unwrap()).collect();
            (numbers[1], numbers[0])
        })
        .fold(HashMap::new(), |mut acc, (key, val)| {
            acc.entry(key).or_insert_with(Vec::new).push(val);
            acc
        });


    let updates: Vec<Vec<u32>> = parts[1]
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn is_valid_update(update: &[u32], rules: &HashMap<u32, Vec<u32>>, pos: usize) -> bool {
    if pos >= update.len() {
        return true;
    }

    if let Some(forward_numbers) = update.get(pos).and_then(|key| rules.get(key)) {
        for to_check in update.iter().skip(pos) {
            if forward_numbers.contains(to_check) {
                return false;
            }
        }
    }
    
    is_valid_update(update, rules, pos + 1)
}


fn make_valid_update(update: &mut Vec<u32>, rules: &HashMap<u32, Vec<u32>>, pos: usize) {
    if pos >= update.len() {
        return;
    }
    
    if let Some(forward_numbers) = update.get(pos).and_then(|key| rules.get(key)) {
        let mut swap_to: Option<usize> = None;
        for (idx, to_check) in update.iter().enumerate().skip(pos) {
            if forward_numbers.contains(to_check) {
                swap_to = Some(idx);
                break;
            }
        }
        
        if let Some(to_index) = swap_to {
            update.swap(to_index, pos);
            return make_valid_update(update, rules, pos);
        }
    }
    
    make_valid_update(update, rules, pos + 1)
}