mod utils;

use utils::vector::Vector;
use std::{collections::{HashSet, VecDeque}, fs, hash::Hash};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

static DIRECTIONS: [(Vector, Direction); 4] = [
    (Vector { x:  0, y: -1 }, Direction::Up),
    (Vector { x:  0, y:  1 }, Direction::Down),
    (Vector { x: -1, y:  0 }, Direction::Left),
    (Vector { x:  1, y:  0 }, Direction::Right),
];


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";
    
    let contents = fs::read_to_string(file_path)?;
    let (grid, size) = parse_input(&contents);

    let mut visited = vec![false; grid.len()];
    let mut plots = Vec::new();

    for (idx, plot) in grid.iter().enumerate() {
        if visited[idx] {
            continue;
        }
        
        plots.push(explore_plot(idx, size, plot, &grid, &mut visited));
    }

    println!("Part One: {}", plots.iter().map(|(area, perimeter, _)| area * perimeter).sum::<usize>());
    println!("Part Two: {}", plots.iter().map(|(area, _, sides)| area * sides).sum::<usize>());

    Ok(())
}

fn explore_plot(idx: usize, size: usize, plot: &char, grid: &Vec<char>, visited: &mut Vec<bool>) -> (usize, usize, usize) {
    let mut stack: VecDeque<usize> = VecDeque::new();
    stack.push_back(idx);
    
    let mut area = 0;
    let mut plot_edges: HashSet<(Vector, Direction)> = HashSet::new();

    while let Some(current) = stack.pop_back() {
        if visited[current] {
            continue;
        }
        visited[current] = true;
        area += 1;

        let (edges, neighbors) = get_sides(current, plot, grid, size);
        for edge in edges {
            plot_edges.insert(edge);
        }

        for neighbor in neighbors {
            stack.push_front(neighbor);
        }
    }

    (area, plot_edges.len(), calculate_sides(&plot_edges))
}

fn calculate_sides(edges: &HashSet<(Vector, Direction)>) -> usize {
    let mut stack: VecDeque<(Vector, Direction)> = VecDeque::new();
    for data in edges.iter() {
        stack.push_back(*data);
    }

    let mut sides = 0;
    while let Some(current) = stack.pop_back() {
        let to_check: Vec<Vector> = match current.1 {
            Direction::Down |
            Direction::Up => [Vector{x: 1, y: 0}, Vector{x: -1, y: 0}].to_vec(),
            Direction::Left |
            Direction::Right => [Vector{x: 0, y: 1}, Vector{x: 0, y: -1}].to_vec()
        };
        
        for dir in to_check {
            let mut cursor = current.clone();
            while let Some(e) = stack.iter().position(|&item| item == (cursor.0 + dir, cursor.1)) {
                cursor.0 = cursor.0 + dir;
                stack.remove(e);
            };
        }

        sides += 1;
    }
    
    sides
}

fn get_sides(index: usize, plot: &char, grid: &[char], size: usize) -> (Vec<(Vector, Direction)>, Vec<usize>) {
    let mut edges: Vec<(Vector, Direction)> = Vec::new();
    let mut neighbors: Vec<usize> = Vec::new();

    let pos = from_array_index(index, size);

    for direction in DIRECTIONS.iter() {
        let new_pos = pos + direction.0;

        if in_bounds(&new_pos, size) {
            let new_index = to_array_index(&new_pos, size);
            if grid[new_index] != *plot {
                edges.push((new_pos, direction.1));
            } else {
                neighbors.push(new_index);
            }
        } else {
            edges.push((new_pos, direction.1));
        }
    }

    (edges, neighbors)
}

fn parse_input(contents: &String) -> (Vec<char>, usize) {
    let grid: Vec<char> = contents.split_whitespace().collect::<Vec<&str>>().join("").chars().collect();
    let size = (grid.len() as f64).sqrt() as usize;
    
    assert!(grid.len() == size.pow(2), "Expected and actual grid size isn't equal!!");

    (grid, size)
}

fn in_bounds(pos: &Vector, size: usize) -> bool {
    pos.x >= 0 && pos.x < size as i32 && pos.y >= 0 && pos.y < size as i32
}

fn to_array_index(pos: &Vector, size: usize) -> usize {
    return pos.x as usize + (size * pos.y as usize);
}

fn from_array_index(index: usize, size: usize) -> Vector {
    let x: i32 = (index % size) as i32;
    let y: i32 = (index / size) as i32;
    
    Vector { x, y }
}
