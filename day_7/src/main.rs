use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";

    let contents = fs::read_to_string(file_path)?;
    let lines = parse_input(&contents);

    let mut all_valid_expressions: Vec<u64> =  vec![0; lines.len()];
    let mut all_valid_expressions_with_concat: Vec<u64> =  vec![0; lines.len()];

    for (idx, line) in lines.iter().enumerate() {
        let split_line: Vec<&str> = line.split(": ").collect();
        let target: u64 = split_line[0].parse::<u64>().unwrap();
        let numbers: Vec<u64> = split_line[1].split(" ").collect::<Vec<&str>>().iter().map(|s| s.parse::<u64>().unwrap()).collect();
        
        if is_valid_expression(target, &numbers, false) {
            all_valid_expressions[idx] = target;
            all_valid_expressions_with_concat[idx] = target;
            continue;
        }
        
        if is_valid_expression(target, &numbers, true) {
            all_valid_expressions_with_concat[idx] = target;
        }
    }
    
    println!("Part One: {}", all_valid_expressions.iter().filter(|&&v| v > 0).sum::<u64>());
    println!("Part Two: {}", all_valid_expressions_with_concat.iter().filter(|&&v| v > 0).sum::<u64>());

    Ok(())
}

fn concatenate_numbers(left: u64, right: u64) -> u64 {
    let right_digits = (10_u64).pow(right.to_string().len() as u32);
    left * right_digits + right
}

fn evaluate_left_to_right(numbers: &[u64], operators: &[char]) -> u64 {
    let mut result = numbers[0];
    
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => result = concatenate_numbers(result, numbers[i + 1]),
            _ => unreachable!(),
        }
    }
   
    result
}


fn is_valid_expression(target: u64, numbers: &Vec<u64>, include_concat: bool) -> bool {
    let possible_sign_positions: u32 = (numbers.len() - 1) as u32;
    let possible_signs: u32 = if include_concat { 3 } else { 2 };

    for combination in 0..(possible_signs.pow(possible_sign_positions)) {
        let mut operators = vec![];
        let mut temp = combination;

        for _ in 0..possible_sign_positions {
            let op = match temp % possible_signs {
                0 => '+',
                1 => '*',
                2 if include_concat => '|',
                _ => unreachable!(),
            };
        
            operators.push(op);
            temp /= possible_signs;
        }

        if evaluate_left_to_right(&numbers, &operators) == target {
            return true;
        }
    }

    false
}

fn parse_input(contents: &String) -> Vec<&str> {
    let lines: Vec<&str> = contents.split("\r\n").collect::<Vec<&str>>();
    lines
}