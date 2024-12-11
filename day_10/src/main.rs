mod vector;
use vector::Vector;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";

    let contents = fs::read_to_string(file_path)?;
    let (grid, trail_heads, size) = parse_input(&contents);
    
    let mut sum_pt1 = 0;
    let mut sum_pt2 = 0;
    for trail_head_pos in trail_heads.iter() {
        let mut visited: Vec<bool> = vec![false; grid.len()];
        sum_pt1 += get_trail_head_score(&grid, &mut visited, *trail_head_pos, true, size);
        sum_pt2 += get_trail_head_score(&grid, &mut visited, *trail_head_pos, false, size);
    }

    println!("Part One: {}", sum_pt1);
    println!("Part Two: {}", sum_pt2);

    Ok(())
}

fn get_trail_head_score(grid: &[u32], visited: &mut Vec<bool>, trail_pos: usize, enable_visited: bool, size: u32) -> u32 {
    let current_elevation = grid[trail_pos];
    
    if current_elevation == 9 {
        if visited[trail_pos] == false || !enable_visited {
            visited[trail_pos] = true;
            return 1;
        } else {
            return 0;
        }
    }

    let valid_neighbors = get_direct_neighbors_of_value(grid, size, trail_pos, current_elevation + 1);
    
    let mut ret = 0;
    for neighbor in valid_neighbors {
        ret += get_trail_head_score(grid, visited, neighbor, enable_visited, size);
    }
    ret
}

fn get_direct_neighbors_of_value(grid: &[u32], size: u32, index: usize, value: u32) -> Vec<usize> {
    let current_pos = from_array_index(index as u32, size);
    let mut direct_neighbors: Vec<usize> = Vec::new();

    let neighbors = [
        Vector { x:  0, y: -1 },
        Vector { x:  0, y:  1 },
        Vector { x: -1, y:  0 },
        Vector { x:  1, y:  0 },
    ];

    for offset in neighbors {
        let to_check = current_pos + offset;
        if in_bounds(&to_check, size) && grid[to_array_index(&to_check, size)] == value {
            direct_neighbors.push(to_array_index(&to_check, size));
        }
    }

    direct_neighbors
}

fn parse_input(contents: &String) -> (Vec<u32>, Vec<usize>, u32) {
    let grid: Vec<u32> = contents.split_whitespace().collect::<Vec<&str>>().join("").chars().map(|c| c.to_digit(10).unwrap()).collect();
    let size = (grid.len() as f64).sqrt() as u32;
    
    let trail_heads: Vec<usize> = grid.iter().enumerate().filter_map(|(idx, num)| {
        if *num == 0 {
            return Some(idx)
        }

        None
    }).collect();

    assert!(grid.len() as u32 == size.pow(2), "Expected and actual grid size isn't equal!!");

    (grid, trail_heads, size)
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
