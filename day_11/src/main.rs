use std::{fs, collections::HashMap};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";

    let contents = fs::read_to_string(file_path)?;
    let stones = parse_input(&contents);
    
    let mut cache: HashMap<(u64, i32), usize> = HashMap::new(); 
    println!("Part One: {:?}", blink_times(stones.clone(), 25, &mut cache));
    println!("Part Two: {:?}", blink_times(stones.clone(), 75, &mut cache));

    Ok(())
}

fn blink_times(stones: Vec<u64>, steps: i32, cache: &mut HashMap<(u64, i32), usize>) -> usize {
    stones.iter().map(|&stone| blink_stone(stone, steps, cache)).sum()
}

fn blink_stone(stone: u64, steps: i32, cache: &mut HashMap<(u64, i32), usize>) -> usize {
    if let Some(&cached_result) = cache.get(&(stone, steps)) {
        return cached_result;
    }

    if steps == 0 {
        return 1;
    }

    let mut result;
    if stone == 0 {
        result = blink_stone(1, steps - 1, cache);
    } else {
        let number_str = stone.to_string();
        if number_str.len() % 2 == 0 {
            let halves = number_str.split_at(number_str.len() / 2);
            result = blink_stone(halves.0.parse().unwrap(), steps - 1, cache) + blink_stone(halves.1.parse().unwrap(), steps - 1, cache);
        } else {
            result = blink_stone(stone * 2024, steps - 1, cache);
        }
    };

    cache.insert((stone, steps), result);
    result
}

fn parse_input(contents: &String) -> Vec<u64> {
    contents.split_whitespace().map(|str| str.parse::<u64>().unwrap()).collect()
}