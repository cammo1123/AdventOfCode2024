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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum CellType {
    None,
    Box,
    Obstacle,
    Robot
}

static DIRECTIONS: [(Vector, Direction); 4] = [
    (Vector { x:  0, y: -1 }, Direction::Up),
    (Vector { x:  0, y:  1 }, Direction::Down),
    (Vector { x: -1, y:  0 }, Direction::Left),
    (Vector { x:  1, y:  0 }, Direction::Right),
];


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./test.txt";
    // let file_path = "./real.txt";
    
    let contents = fs::read_to_string(file_path)?;
    let (grid, size, moves) = parse_input(&contents);
    let robots = grid.iter().enumerate().filter_map(|(idx, &c)| {if c == CellType::Robot { return Some(idx) } None}).collect::<Vec<usize>>();
    let robot = robots.first().unwrap();

    println!("{:?}", from_array_index(*robot, size));
    Ok(())
}

fn parse_input(contents: &String) -> (Vec<CellType>, usize, Vec<Direction>) {
    let parts: Vec<&str> = contents.split("\r\n\r\n").collect();
    

    let grid_lines: Vec<&str> = parts.get(0).unwrap().split("\r\n").collect();
    let mut grid: Vec<CellType> = Vec::new();

    for (idx, line) in grid_lines.iter().enumerate() {
        if idx == 0 || idx == grid_lines.len() - 1 {
            continue;
        }

        let sanitized = line.strip_prefix("#").unwrap().strip_suffix("#").unwrap();
        for char in sanitized.chars() {
            grid.push(match char {
                '.' => CellType::None,
                'O' => CellType::Box,
                '#' => CellType::Obstacle,
                '@' => CellType::Robot,
                _ => unreachable!()
            });
        }
    }

    let size = (grid.len() as f64).sqrt() as usize;
    assert!(grid.len() == size.pow(2), "Expected and actual grid size isn't equal!!");
    
    let moves: Vec<Direction> = parts.get(1).unwrap().split("\r\n").collect::<Vec<&str>>().join("").chars().map(|c| {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unreachable!()
        }
    }).collect();
    
    (grid, size, moves)
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
