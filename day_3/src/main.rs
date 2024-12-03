use std::fs;

fn main() {
	// let file_path = "./test.txt";
	let file_path = "./real.txt";

	let contents = fs::read_to_string(file_path)
		.expect("Unable to read file");

    println!("Part One: {}", check_memory(contents.clone(), false));
    println!("Part Two: {}", check_memory(contents.clone(), true));
}

fn check_memory(contents: String, enable_instructions: bool) -> u32 {
    let mut letter = contents.chars().peekable();
    let mut index = 0;
    let mut running: u32 = 0;
    let mut enabled: bool = true;

    while letter.peek() != None {
        let mut char = letter.next().unwrap();

        if char == 'd' && enable_instructions {
            if &contents[index..index + 2] == "do" {
                letter.next();
                letter.next();
                index += 2;

                if &contents[index..index + 2] == "()" {
                    letter.next();
                    index += 1;

                    enabled = true;
                } 
                
                if &contents[index..index + 4] == "n't(" {
                    letter.next();
                    letter.next();
                    letter.next();
                    index += 3;

                    enabled = false;
                }
            }
        }
        
        if char == 'm' && index + 8 <= contents.len() && enabled {
            if &contents[index..index + 4] == "mul(" {
                letter.next();
                letter.next();
                letter.next();
                index += 3;

                let mut maybe_num_one = extract_number(&mut letter, &mut index);
                if maybe_num_one == None {
                    continue;
                }
                let num_one = maybe_num_one.take().unwrap();

                char = letter.next().unwrap();
                index += 1;    
                if char != ',' {
                    index += 1;
                    continue;
                }
            
                let mut maybe_num_two = extract_number(&mut letter, &mut index);
                if maybe_num_two == None {
                    continue;
                }
                let num_two = maybe_num_two.take().unwrap();

                char = letter.next().unwrap();
                index += 1;    
                if char != ')' {
                    index += 1;
                    continue;
                }

                running += num_one * num_two;
            }
        }
        index += 1;
    }

    return running;
}

fn extract_number(letter: &mut std::iter::Peekable<std::str::Chars<'_>>, index: &mut usize) -> Option<u32> {
    let mut number = 0;

    while let Some(&ch) = letter.peek() {
        if ch.is_digit(10) {
            let ch = letter.next().unwrap(); // Consume the character
            *index += 1;
            number = number * 10 + ch.to_digit(10).unwrap();
        } else {
            break;
        }
    }

    return Some(number);
}
