use std::fs;

fn main() {
    // let file_path = "./test.txt";
	let file_path = "./real.txt";

	let contents = fs::read_to_string(file_path)
		.expect("Unable to read file");

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = contents
		.split("\n")
		.map(|line| { 
            let mut split_line = line.split_whitespace();
            let left = split_line.next().unwrap().parse::<i32>().unwrap();
            let right = split_line.next().unwrap().parse::<i32>().unwrap();

            return (left, right);
        })
		.collect::<(Vec<i32>, Vec<i32>)>();

    left.sort_unstable();
    right.sort_unstable();

    // Part 1
    let mut running: i64 = 0;

    for i in 0..left.len() {
        let diff: i32 = left[i] - right[i];
        running += diff.abs() as i64;
    }
    
    println!("Part One: {}", running);

    // Part 2
    let mut current_number: i32 = 0;
    running = 0;

    for i in 0..left.len() {
        if left[i] != current_number {
            current_number = left[i];
            let count = right.iter().filter(|&v| *v == current_number).count() as i64;
            running += (current_number as i64) * count;
        }
    }

    println!("Part Two: {}", running);
}