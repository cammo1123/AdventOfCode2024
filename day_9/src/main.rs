use std::fs;

#[derive(Debug, Clone)]
struct Block {
    id: Option<u64>,
    size: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./test.txt";
    // let file_path = "./real.txt";

    let contents = fs::read_to_string(file_path)?;
    let (basic, advanced) = parse_input(&contents);
    
    let mut usage_vec = basic.clone();
    let mut compact = Vec::<u64>::new();
    let mut last_moved_index = usage_vec.len();

    for (index, value) in basic.iter().enumerate() {
        if index >= last_moved_index {
            break;
        }

        match value {
            Some(value) => {
                compact.push(*value);
            },
            None => {
                let maybe_block = usage_vec
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, val)| val.is_some());
                
                if let Some((index, value)) = maybe_block {
                    compact.push(value.unwrap());
                    usage_vec.remove(index);
                    last_moved_index = index;
                }
            }
        }
    }

    let mut checksum = 0;
    for (index, value) in compact.iter().enumerate() {
        checksum += index as u64 * value;
    }

    
    let mut adjusted = advanced.clone();
    for block in advanced.iter().rev() {
        if block.id.is_none() { continue }

        let maybe_first_to_fit: Option<usize> = adjusted
            .iter()
            .enumerate()
            .filter(|(_, v)| v.id.is_none() && v.size >= block.size)
            .map(|(idx, _)| idx)
            .next();

        let maybe_block_index = adjusted
            .iter()
            .enumerate()
            .filter(|(_, s)| s.id.is_some() && s.id == block.id)
            .map(|(s,_)| s)
            .next();
    
        if let Some(first_to_fit) = maybe_first_to_fit {
            if let Some(block_index) = maybe_block_index {
                if block_index <= first_to_fit {
                    break;
                }

                adjusted[first_to_fit].size -= block.size;
                adjusted[block_index] = Block { id: None, size: block.size };
                adjusted.insert(first_to_fit, Block {id: block.id, size: block.size});
            }
        }
    }
    
    println!("Part One: {}", checksum);
    println!("Part Two: {}", calculate_block_vec_checksum(adjusted));

    Ok(())
}

fn calculate_block_vec_checksum(adjusted: Vec<Block>) -> u64{
    let mut checksum = 0;
    let mut start = 0;
    
    for block in adjusted.iter() {
        if let Some(id) = block.id {
            for idx in start..start + block.size {
                checksum += idx as u64 * id;
            }
        }
    
        start += block.size;
    }

    checksum
}

fn parse_input(contents: &String) -> (Vec<Option<u64>>, Vec<Block>) {
    let mut mode: u8 = 0;
    let mut index: u64 = 0;

    let basic = contents.chars().map(|cha| cha.to_digit(10).unwrap())
        .fold(Vec::<Option<u64>>::new(), |mut acc, x| {
            match mode {
                0 => {
                    for _ in 0..x {
                        acc.push(Some(index));
                    }
                    mode = 1;
                    index += 1;
                },
                1 => {
                    for _ in 0..x {
                        acc.push(None);
                    }
                    mode = 0;
                }
                _ => unreachable!()
            }

            acc
        });

    index = 0;
    mode = 0;
    let advanced = contents.chars().map(|cha| cha.to_digit(10).unwrap())
        .fold(Vec::<Block>::new(), |mut acc, x| {
            match mode {
                0 => {
                    acc.push(Block{ id: Some(index), size: x as u64});

                    mode = 1;
                    index += 1;
                },
                1 => {
                    acc.push(Block{ id: None, size: x as u64});
                    mode = 0;
                }
                _ => unreachable!()
            }

            acc
        });

    

    (basic, advanced)
}