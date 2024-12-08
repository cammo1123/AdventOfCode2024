mod vector;
use vector::Vector;

use std::{collections::HashMap, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";
    let contents = fs::read_to_string(file_path)?;
    let (map, grid, size) = parse_input(&contents);
   
    let mut visited_pt1 = vec![false; grid.len()];
    let mut visited_pt2 = visited_pt1.clone();

    for (_, vec) in map.iter() {
        for pos1 in vec {
            for pos2 in vec {
                if pos1 == pos2 { continue }
                let diff = *pos2 - *pos1;
                
                if in_bounds(&(*pos2 + diff), size) {
                    visited_pt1[to_array_index(&(*pos2 + diff), size) as usize] = true;
                }
                
                let mut new_pos = *pos2;
                while in_bounds(&new_pos, size) {                
                    visited_pt2[to_array_index(&new_pos, size) as usize] = true;
                    new_pos += diff;
                }
            }
        }
    }
    
    println!("Part One: {}", visited_pt1.iter().filter(|&&val| val == true).count());
    println!("Part One: {}", visited_pt2.iter().filter(|&&val| val == true).count());

    Ok(())
}

fn parse_input(contents: &String) -> (HashMap<char, Vec<Vector>>, Vec<char>, u32) {
    let grid: Vec<char> = contents.split_whitespace().collect::<Vec<&str>>().join("").chars().collect();
    let size = (grid.len() as f64).sqrt() as u32;

    let map: HashMap<char, Vec<Vector>> = grid.iter().enumerate()
        .fold(HashMap::new(), |mut map, (index, &char)| {
            if char != '.' {
                let vector = from_array_index(index as u32, size);
                map.entry(char).or_insert_with(Vec::new).push(vector);
            }

            map
        });

    assert!(grid.len() as u32 == size.pow(2), "Expected and actual grid size isn't equal!!");

    (map, grid, size)
}

fn in_bounds(pos: &Vector, size: u32) -> bool {
    pos.x >= 0 && pos.x < size as i32 && pos.y >= 0 && pos.y < size as i32
}

fn to_array_index(pos: &Vector, size: u32) -> usize {
    return pos.x as usize + (size as usize * pos.y as usize);
}

fn from_array_index(index: u32, size: u32) -> Vector {
    let x: i32 = (index % size) as i32;
    let y: i32 = (index / size) as i32;
    
    Vector { x, y }
}
