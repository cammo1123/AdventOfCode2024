mod utils;

use utils::vector::Vector;
use std::{fs, hash::Hash, process::exit};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";
    
    let contents = fs::read_to_string(file_path)?;
    let (grid, size, moves) = parse_input(&contents);
    let Some((robot_pos, _)) = grid.iter().enumerate().find(|(_, &cell)| cell == CellType::Robot) else {
        exit(0);
    };

    let mut pt1_grid = grid.clone();
    let mut pt1_robot_pos = robot_pos.clone();
    for robot_move in moves.iter() {
        let robot = from_array_index(pt1_robot_pos, size);
        let movable = get_movable(robot, &vector_from_direction(robot_move), &mut pt1_grid, size);
        
        if let Some(movable_boxes) = movable {
            let last_pos = to_array_index(movable_boxes.last().unwrap(), size);
            let first_pos = to_array_index(movable_boxes.first().unwrap(), size);
            
            pt1_grid.swap(first_pos, last_pos);
            pt1_grid.swap(first_pos, pt1_robot_pos);
            pt1_robot_pos = first_pos;
        }
    }
    
    let gps_sum: usize = pt1_grid.iter().enumerate()
        .filter_map(|(pos, &cell)| {
            if cell != CellType::Box { return None }
            let vec = from_array_index(pos, size);
            Some((100 * vec.y as usize) + vec.x as usize)
        })
        .sum();

    println!("Part One: {}", gps_sum);
    Ok(())
}

fn print_grid(grid: &Vec<CellType>, size: usize) {
    for y in 0..size {
        for x in 0..size {
            let cell = grid[y * size + x];
            let char = match cell {
                CellType::None => '.',
                CellType::Box => 'O',
                CellType::Obstacle => '#',
                CellType::Robot => '@',
            };
            print!("{}", char);
        }
        println!();
    }
}

fn get_movable(robot: Vector, robot_move: &Vector, grid: &mut Vec<CellType>, size: usize) -> Option<Vec<Vector>> {
    let mut look_ahead = robot + *robot_move;

    let mut ahead: Vec<Vector> = Vec::new();
    while in_bounds(&look_ahead, size) {
        let cell = grid[to_array_index(&look_ahead, size)];
        
        if cell == CellType::None {
            ahead.push(look_ahead);
            return Some(ahead);
        }
        
        if cell == CellType::Obstacle {
            return None;
        }
        
        ahead.push(look_ahead);
        look_ahead += *robot_move;
    }

    None
}

fn vector_from_direction(direction: &Direction) -> Vector {
    match direction {
        Direction::Up => Vector { x:  0, y: -1 },
        Direction::Down => Vector { x:  0, y:  1 },
        Direction::Left => Vector { x: -1, y:  0 },
        Direction::Right => Vector { x:  1, y:  0 },
    }
}

fn parse_input(contents: &String) -> (Vec<CellType>, usize, Vec<Direction>) {
    let parts: Vec<String> = contents.split("\r\n\r\n").map(|part| part.split("\r\n").collect::<Vec<&str>>().join("")).collect();

    let grid: Vec<CellType> = parts.get(0).unwrap().chars().map(|cell| match cell {
        '.' => CellType::None,
        'O' => CellType::Box,
        '#' => CellType::Obstacle,
        '@' => CellType::Robot,
        _ => unreachable!()
    }).collect();
    
    let moves: Vec<Direction> = parts.get(1).unwrap().chars().map(|instruction| match instruction {
        '<' => Direction::Left,
        '>' => Direction::Right,
        '^' => Direction::Up,
        'v' => Direction::Down,
        _ => unreachable!()
    }).collect();

    let size = (grid.len() as f64).sqrt() as usize;
    assert!(grid.len() == size.pow(2), "Expected and actual grid size isn't equal!!");

    (grid, size, moves)
}

fn in_bounds(pos: &Vector, size: usize) -> bool {
    pos.x >= 0 && pos.x < size as i64 && pos.y >= 0 && pos.y < size as i64
}

fn to_array_index(pos: &Vector, size: usize) -> usize {
    return pos.x as usize + (size * pos.y as usize);
}

fn from_array_index(index: usize, size: usize) -> Vector {
    let x: i64 = (index % size) as i64;
    let y: i64 = (index / size) as i64;
    
    Vector { x, y }
}
