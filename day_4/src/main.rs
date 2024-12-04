use std::{fs, usize};

fn main() {
    // let file_path = "./test.txt";
    let file_path = "./real.txt";

    let contents = fs::read_to_string(file_path).expect("Unable to read file");

    let letters: Vec<char> = contents
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .chars()
        .collect();
    let size = (letters.len() as f64).sqrt() as usize;

    println!("Part One: {}", part_one(&letters, size));
    println!("Part Two: {}", part_two(&letters, size));
}

fn part_two(letters: &[char], size: usize) -> usize {
    let mut aggr = 0;

    for (idx, &letter) in letters.iter().enumerate() {
        if letter == 'A' {
            let (x, y) = from_array_index(idx, size);

            if let Some(adj) = get_adjacent(letters, x, y, size) {
                let [top_left, top_right, bottom_left, bottom_right] = adj;

                if is_valid_corner(top_left, bottom_right)
                    && is_valid_corner(top_right, bottom_left)
                {
                    aggr += 1;
                }
            }
        }
    }

    aggr
}

fn part_one(letters: &[char], size: usize) -> usize {
    let xmas_letters = "XMAS".chars().collect::<Vec<char>>();
    let directions = [
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    letters.iter().enumerate().fold(0, |mut acc, (idx, _)| {
        for &dir in &directions {
            acc += check_sequence(letters, idx, size, 0, &xmas_letters, dir);
        }
        acc
    })
}

fn check_sequence(
    letters: &[char],
    index: usize,
    size: usize,
    pos: usize,
    sequence: &[char],
    dir: (i32, i32),
) -> usize {
    let (x, y) = from_array_index(index, size);

    if letters.get(index) != sequence.get(pos) {
        return 0;
    }

    if pos == sequence.len() - 1 {
        return 1;
    }

    if !in_bounds(x + dir.0, y + dir.1, size) {
        return 0;
    }

    check_sequence(
        letters,
        to_array_index(x + dir.0, y + dir.1, size),
        size,
        pos + 1,
        sequence,
        dir,
    )
}

fn is_valid_corner(a: char, b: char) -> bool {
    (a == 'M' && b == 'S') || (a == 'S' && b == 'M')
}

fn get_adjacent(letters: &[char], x: i32, y: i32, size: usize) -> Option<[char; 4]> {
    let coords = [
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y + 1),
    ];

    coords
        .iter()
        .map(|&(nx, ny)| {
            if in_bounds(nx, ny, size) {
                letters.get(to_array_index(nx, ny, size)).cloned()
            } else {
                None
            }
        })
        .collect::<Option<Vec<char>>>()
        .and_then(|v| v.try_into().ok())
}

fn in_bounds(x: i32, y: i32, size: usize) -> bool {
    x >= 0 && x < size as i32 && y >= 0 && y < size as i32
}

fn to_array_index(x: i32, y: i32, size: usize) -> usize {
    return x as usize + (size * y as usize);
}

fn from_array_index(index: usize, size: usize) -> (i32, i32) {
    let x = index % size;
    let y = index / size;
    (x as i32, y as i32)
}
