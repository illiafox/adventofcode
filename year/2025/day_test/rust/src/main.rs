use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Lines;

struct Input {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn read_input(lines: Lines) -> Input {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let parts: Vec<i32> = line
            .split("   ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if parts.len() != 2 {
            panic!("invalid input file")
        }

        left.push(parts[0]);
        right.push(parts[1]);
    }

    left.sort();
    right.sort();

    Input { left, right }
}

fn part_one(input: &Input) -> i32 {
    let mut total_distance = 0;
    for i in 0..input.left.len() {
        total_distance += i32::abs(input.left[i] - input.right[i])
    }

    total_distance
}

fn part_two(input: &Input) -> i32 {
    let mut occurrences: HashMap<i32, i32> = HashMap::new();

    for &num in input.right.iter() {
        *occurrences.entry(num).or_default() += 1
    }

    let mut similarity_score = 0;
    for l in input.left.iter() {
        similarity_score += i32::abs(l * occurrences.get(l).copied().unwrap_or_default())
    }

    similarity_score
}

fn main() {
    let filename = "../testdata/input.txt";
    let file = read_to_string(filename).unwrap();

    let input = read_input(file.lines());

    let part_one = part_one(&input);
    println!("part 1: total distance: {part_one}");

    let part_two = part_two(&input);
    println!("part 2: similarity score: {part_two}");
}

#[test]
fn test() {
    let file = include_str!("../../testdata/input.txt");
    let input = read_input(file.lines());

    assert_eq!(part_one(&input), 2580760);
    assert_eq!(part_two(&input), 25358365);
}
