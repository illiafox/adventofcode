use std::collections::{HashMap, HashSet};

struct Input {
    numbers: Vec<Vec<i64>>,
    operators: Vec<char>,
}

fn read_input(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn traverse(
    grid: &Vec<Vec<char>>,
    point: (usize, usize),
    splits: &mut HashSet<(usize, usize)>,
) -> i64 {
    let (start_i, start_j) = point;

    let mut splits_total = 0;

    for i in start_i..grid.len() {
        if grid[i][start_j] == '^' {
            let mut new_branch = false;

            let left = (i, start_j - 1);
            if splits.insert(left) {
                new_branch = true;
                splits_total += traverse(grid, left, splits);
            }

            let right = (i, start_j + 1);
            if splits.insert(right) {
                new_branch = true;
                splits_total += traverse(grid, right, splits);
            }

            if new_branch {
                splits_total += 1;
            }

            break;
        }
    }

    splits_total
}

fn start_position(grid: &[Vec<char>]) -> (usize, usize) {
    let start_j = grid[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("no 'S' in first row");
    (1, start_j)
}

fn part_one(grid: &Vec<Vec<char>>) -> i64 {
    traverse(grid, start_position(grid), &mut HashSet::new())
}

fn traverse_quant(
    grid: &Vec<Vec<char>>,
    point: (usize, usize),
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if let Some(&splits_total) = memo.get(&point) {
        return splits_total;
    }

    let (start_i, start_j) = point;

    let mut splits_total = 0;

    for i in start_i..grid.len() {
        if grid[i][start_j] == '^' {
            let left = (i, start_j - 1);
            splits_total += traverse_quant(grid, left, memo);

            let right = (i, start_j + 1);
            splits_total += traverse_quant(grid, right, memo);

            memo.insert(point, splits_total);
            return splits_total;
        }
    }

    splits_total += 1;

    memo.insert(point, splits_total);
    splits_total
}

fn part_two(grid: &Vec<Vec<char>>) -> i64 {
    traverse_quant(grid, start_position(grid), &mut HashMap::new())
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let input = read_input(file);

    assert_eq!(part_one(&input), 21);
    assert_eq!(part_two(&input), 40);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let input = read_input(file);

    assert_eq!(part_one(&input), 1507);
    assert_eq!(part_two(&input), 1537373473728);
}
