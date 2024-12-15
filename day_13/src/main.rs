mod utils;

use utils::vector::Vector;
use std::{fs, hash::Hash};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct ClawMachine {
    button_a: Vector,
    button_b: Vector,
    prize: Vector,
}

impl ClawMachine {
    fn cheapest(&self) -> u64 {
        let ((gx, gy), (ax, ay), (bx, by)) 
            = ((self.prize.x, self.prize.y), (self.button_a.x, self.button_a.y), (self.button_b.x, self.button_b.y));

        // The best a and b values for a case are the solutions to the following
        // system of equations, where g is the goal position, a/b are the number
        // of times you press the corresponding buttons (what we are solving
        // for), and (a|b)(x|y) are the offsets applied by each button press.
        //
        // gx = ax * a + bx * b
        // gy = ay * a + by * b
        //
        // By solving these equations algebraically, we get the formula for a and b.
        // We check if a and b are valid (positive and the solution matches the goal).
        
        let a = (by * gx - bx * gy) / (ax * by - ay * bx);
        let b = (gx - ax * a) / bx;

        if !a.is_positive() || !b.is_positive() {
            return 0;
        }

        if self.prize != self.button_a.scale(a as i64) + self.button_b.scale(b as i64) {
            return 0;
        }

        a as u64 * 3 + b as u64
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";
    
    let contents = fs::read_to_string(file_path)?;
    let tests = parse_input(&contents);

    
    let mut pt2_tests = tests.clone();
    pt2_tests.iter_mut().for_each(|x| x.prize = x.prize + Vector{x: 10000000000000, y: 10000000000000});
    
    println!("Part One: {}", tests.iter().map(|x| x.cheapest()).sum::<u64>());
    println!("Part Two: {}", pt2_tests.iter().map(|x| x.cheapest()).sum::<u64>());

    Ok(())
}

fn parse_input(contents: &String) -> Vec<ClawMachine> {
    let grid = contents.split("\r\n\r\n").map(|machine| {
        let parts: Vec<&str> = machine.split("\r\n").collect();
        let mut button_a: Option<Vector> = None;
        let mut button_b: Option<Vector> = None;
        let mut prize: Option<Vector> = None;

        for (idx, part) in parts.iter().enumerate() {
            let Some(button_cost) = part.split(": ").nth(1) else {
                continue;
            };

            let mut cost_sides: Vec<&str> = button_cost.split(", ").collect();

            if idx <= 1 {
                let x = cost_sides[0].strip_prefix("X").unwrap().parse::<i64>().unwrap();
                let y = cost_sides[1].strip_prefix("Y").unwrap().parse::<i64>().unwrap();
                
                match idx {
                    0 => button_a = Some(Vector{x, y}),
                    1 => button_b = Some(Vector{x, y}),
                    _ => unreachable!()
                }
            } else {
                let x = cost_sides[0].strip_prefix("X=").unwrap().parse::<i64>().unwrap();
                let y = cost_sides[1].strip_prefix("Y=").unwrap().parse::<i64>().unwrap();
                prize = Some(Vector{x, y});
            }
        }

        ClawMachine { button_a: button_a.unwrap(), button_b: button_b.unwrap(), prize: prize.unwrap() }
    }).collect();

    grid
}
