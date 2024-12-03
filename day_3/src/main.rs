use std::fs;
use regex::Regex;

fn main() {
	// let file_path = "./test.txt";
	let file_path = "./real.txt";

	let contents = fs::read_to_string(file_path)
		.expect("Unable to read file");

    println!("Part One: {}", check_memory(&contents, false));
    println!("Part Two: {}", check_memory(&contents, true));
}

fn check_memory(contents: &str, respect_instructions: bool) -> u32 {
    let command_regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut total = 0;

    for capture in command_regex.captures_iter(contents) {
        if let Some(cmd) = capture.get(0) {
            match cmd.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ if cmd.as_str().starts_with("mul") => {
                    if enabled || !respect_instructions {
                        let params = cmd.as_str()[4..cmd.as_str().len() - 1].split(',');
                        let sides: Vec<u32> = params.map(|f| f.parse::<u32>().unwrap()).collect();
                        let (left, right) = (sides[0], sides[1]);

                        total += left * right;
                    }
                }
                _ => {}
            }
        }
    }

    total
}