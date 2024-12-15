mod utils;

use utils::vector::Vector;
use std::{fs, hash::Hash, process::id};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    // let width = 11;
    // let height = 7;

    let file_path = "./real.txt";
    let width = 101;
    let height = 103;
    
    let contents = fs::read_to_string(file_path)?;
    let robots = parse_input(&contents);

    let mut pt1_robots = robots.clone();
    for robot in pt1_robots.iter_mut() {
        move_steps(100, robot, width, height);
    }

    let (q1, q2, q3, q4) = calculate_safety(&pt1_robots, width, height);
    let mut pt2_robots = robots.clone();
    for robot in pt2_robots.iter_mut() {
        move_steps(7055, robot, width, height);
    }
    display_grid(&pt2_robots, width, height);
    
    println!("Part One: {}", q1 * q2 * q3 * q4);
    println!("Part Two: 7055"); // Found Manually


    Ok(())
}

fn display_grid(robots: &Vec<Robot>, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            let number = robots.iter().filter(|r| r.position.x == x && r.position.y == y).count();
            
            if number == 0 {
                print!(".")
            } else {
                print!("{number}")
            }
        }

        println!("")
    }
}

fn calculate_safety(robots: &Vec<Robot>, width: i64, height: i64) -> (usize, usize, usize, usize) {
    (
        robots.iter().filter(|r| r.position.x < width / 2 && r.position.y > height / 2).collect::<Vec<&Robot>>().len(),
        robots.iter().filter(|r| r.position.x < width / 2 && r.position.y < height / 2).collect::<Vec<&Robot>>().len(),
        robots.iter().filter(|r| r.position.x > width / 2 && r.position.y > height / 2).collect::<Vec<&Robot>>().len(),
        robots.iter().filter(|r| r.position.x > width / 2 && r.position.y < height / 2).collect::<Vec<&Robot>>().len()
    )
}

fn move_steps(steps: usize, robot: &mut Robot, width: i64, height: i64) {
    robot.position += robot.velocity.scale(steps as i64);

    robot.position.x = ((robot.position.x % width) + width) % width;
    robot.position.y = ((robot.position.y % height) + height) % height;
}

fn parse_input(contents: &String) -> Vec<Robot> {
    let grid = contents.split("\r\n").map(|robot| {
        let parts: Vec<&str> = robot.split_whitespace().collect();
        let pos_raw = parts.get(0).unwrap().strip_prefix("p=").unwrap();
        let vel_raw = parts.get(1).unwrap().strip_prefix("v=").unwrap();

        
        let pos_parts: Vec<&str> = pos_raw.split(",").collect();
        let pos_x = pos_parts.get(0).unwrap().parse::<i64>().unwrap();
        let pos_y = pos_parts.get(1).unwrap().parse::<i64>().unwrap();

        let vel_parts: Vec<&str> = vel_raw.split(",").collect();
        let vel_x = vel_parts.get(0).unwrap().parse::<i64>().unwrap();
        let vel_y = vel_parts.get(1).unwrap().parse::<i64>().unwrap();

        Robot{ position: Vector{ x: pos_x, y: pos_y }, velocity: Vector { x: vel_x, y: vel_y }}
    }).collect::<Vec<Robot>>();

    grid
}
