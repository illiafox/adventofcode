use std::collections::{HashMap, HashSet};

fn parse_grid(s: &str) -> Vec<Vec<char>> {
    s.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn next_split_row(grid: &[Vec<char>], start_i: usize, col: usize) -> Option<usize> {
    (start_i..grid.len()).find(|&i| grid[i][col] == '^')
}

fn split_directions(i: usize, j: usize) -> [(usize, usize); 2] {
    [(i, j - 1), (i, j + 1)]
}

fn traverse_unique(
    grid: &[Vec<char>],
    point: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
) -> i64 {
    let (start_i, j) = point;

    let Some(i) = next_split_row(grid, start_i, j) else {
        return 0;
    };

    let mut subtotal = 0;
    let mut new_branch = false;

    for direction in split_directions(i, j) {
        if seen.insert(direction) {
            new_branch = true;
            subtotal += traverse_unique(grid, direction, seen);
        }
    }

    if new_branch { subtotal + 1 } else { subtotal }
}

fn start_position(grid: &[Vec<char>]) -> (usize, usize) {
    let start_j = grid[0]
        .iter()
        .position(|&c| c == 'S')
        .expect("no 'S' in first row");
    (1, start_j)
}

fn part_one(grid: &[Vec<char>]) -> i64 {
    traverse_unique(grid, start_position(grid), &mut HashSet::new())
}

fn traverse_quant(
    grid: &[Vec<char>],
    point: (usize, usize),
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if let Some(&subtotal) = memo.get(&point) {
        return subtotal;
    }

    let (start_i, j) = point;

    let subtotal = if let Some(i) = next_split_row(grid, start_i, j) {
        let [left, right] = split_directions(i, j);
        traverse_quant(grid, left, memo) + traverse_quant(grid, right, memo)
    } else {
        1 // grid end
    };

    memo.insert(point, subtotal);
    subtotal
}

fn part_two(grid: &[Vec<char>]) -> i64 {
    traverse_quant(grid, start_position(grid), &mut HashMap::new())
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let grid = parse_grid(file);

    assert_eq!(part_one(&grid), 21);
    assert_eq!(part_two(&grid), 40);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let grid = parse_grid(file);

    assert_eq!(part_one(&grid), 1507);
    assert_eq!(part_two(&grid), 1537373473728);
}
