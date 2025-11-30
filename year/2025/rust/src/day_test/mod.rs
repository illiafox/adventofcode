use std::collections::HashMap;
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

fn get_total_distance(input: &Input) -> i32 {
    let mut total_distance = 0;
    for i in 0..input.left.len() {
        total_distance += i32::abs(input.left[i] - input.right[i])
    }

    total_distance
}

fn get_similarity_score(input: &Input) -> i32 {
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

#[test]
fn test() {
    let file = include_str!("input.txt");
    let input = read_input(file.lines());

    let total_distance = get_total_distance(&input);
    let similarity_score = get_similarity_score(&input);

    println!("part 1: total distance: {total_distance}");
    println!("part 2: similarity score: {similarity_score}");

    assert_eq!(total_distance, 2580760);
    assert_eq!(similarity_score, 25358365);
}
