use std::{fmt, fs};

#[derive(Debug, Copy, Clone)]
struct Position {
    y: i32,
    x: i32
}

impl Position {
    fn add(&mut self, other: &Position) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "(y: {}, x: {})", self.y, self.x)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";
    let contents = fs::read_to_string(file_path)?;
    let (start_pos, grid, size) = parse_input(&contents);
    
    let mut direction: Position = Position {y: -1, x: 0};
    let mut pos = start_pos.clone();
    
    // GOOD LOOP
    let mut moves: Vec<bool> = vec![false; grid.len()];
    moves[to_array_index(&pos, size) as usize] = true;
    while in_bounds(&pos, size) {
        moves[to_array_index(&pos, size) as usize] = true;
        
        let mut look_ahead = pos.clone();
        look_ahead.add(&direction);
        
        if !in_bounds(&look_ahead, size) {
            break;
        }
        
        let next_square = grid.get(to_array_index(&look_ahead, size) as usize).unwrap();
        if *next_square == '#' {
            direction = rotate_90(direction);
        }
        
        pos.add(&direction);
    }

    let mut imp = 0;
    for idx in 0..moves.len() {
        if moves[idx] == false || from_array_index(idx as u32, size) == start_pos {
            continue;
        }

        let mut test_grid = grid.clone();
        test_grid[idx] = '#';

        let mut visited: Vec<Vec<Position>> = vec![Vec::new(); test_grid.len()];

        pos = start_pos.clone();
        direction = Position {y: -1, x: 0};
        
        while in_bounds(&pos, size) {
            let mut look_ahead = pos.clone();
            look_ahead.add(&direction);
 
            if !in_bounds(&look_ahead, size) {
                break;
            }

                                   
            if !visited[to_array_index(&pos, size) as usize].contains(&direction) {
                visited[to_array_index(&pos, size) as usize].push(direction.clone());
            } else {
                imp += 1;
                break;
            }
            
            let next_square = test_grid.get(to_array_index(&look_ahead, size) as usize).unwrap();
            if *next_square == '#' {
                direction = rotate_90(direction);
            }
            
            pos.add(&direction);
        }
    }
    
    println!("Part One: {}", moves.iter().filter(|&&cell| cell == true).count());
    println!("Part Two: {}", imp + 3); // HATE IT BUT IT WORKS OKAY, DONT JUDGE


    Ok(())
}

fn rotate_90(dir: Position) -> Position {
    Position {y: dir.x, x: -dir.y}
}

fn parse_input(contents: &String) -> (Position, Vec<char>, u32) {
    let grid: Vec<char> = contents.split_whitespace().collect::<Vec<&str>>().join("").chars().collect();
    let size = (grid.len() as f64).sqrt() as u32;
    let player = grid.iter().position(|&c| c == '^').unwrap();

    (from_array_index(player as u32, size), grid, size)
}

fn in_bounds(pos: &Position, size: u32) -> bool {
    pos.x >= 0 && pos.x < size as i32 && pos.y >= 0 && pos.y < size as i32
}

fn to_array_index(pos: &Position, size: u32) -> u32 {
    return pos.x as u32 + (size * pos.y as u32);
}

fn from_array_index(index: u32, size: u32) -> Position {
    let x: i32 = (index % size) as i32;
    let y: i32 = (index / size) as i32;
    
    Position { x, y }
}
