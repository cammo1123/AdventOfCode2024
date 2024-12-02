use std::fs;

fn main() {
	// let file_path = "./test.txt";
	let file_path = "./real.txt";

	let contents = fs::read_to_string(file_path)
		.expect("Unable to read file");

	let mapped_lines: Vec<Vec<i32>> = contents
		.split("\n")
		.map(|line| line.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect())
		.collect();

	println!("Part One: {}", mapped_lines.iter().filter(|line| is_safe(line, false)).count());
	println!("Part Two: {}", mapped_lines.iter().filter(|line| is_safe(line, true)).count());
}

fn is_safe(line: &[i32], can_remove: bool) -> bool {
	let line_direction = (line[1] - line[0]).signum();

	for i in 0..line.len() - 1 {
		let mut diff = line[i + 1] - line[i];
		let diff_sign = diff.signum();
		diff = diff.abs();

		if line_direction == 0 || diff_sign != line_direction || diff > 3 || diff < 1 {
			if can_remove {
				return line.iter()
					.enumerate()
					.any(|(j, _)| is_safe(&remove_at(line, j), false));
			}

			return false;
		}
	}

	return true;
}


fn remove_at(line: &[i32], index: usize) -> Vec<i32> {
    let mut new_line = line.to_vec();
    new_line.remove(index);
    new_line
}