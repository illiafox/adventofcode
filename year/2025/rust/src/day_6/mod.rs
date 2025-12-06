struct Input {
    numbers: Vec<Vec<i64>>,
    operators: Vec<char>,
}

fn read_input(s: &str) -> Input {
    let mut rows = s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>();

    let op_line = rows.pop().unwrap();

    let numbers = rows
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let operators = op_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    Input { numbers, operators }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn get_op(c: char) -> (fn(i64, i64) -> i64, i64) {
    match c {
        '+' => (|a, b| a + b, 0),
        '*' => (|a, b| a * b, 1),
        _ => panic!("unsupported char"),
    }
}

fn part_one(numbers: Vec<Vec<i64>>, operators: Vec<char>) -> i64 {
    let mut sum = 0;

    for j in 0..operators.len() {
        let (op, mut start) = get_op(operators[j]);

        for row in numbers.iter() {
            start = op(start, row[j])
        }

        sum += start
    }

    sum
}

fn part_two(file: &str) -> i64 {
    let grid = parse_grid(file);
    let last = grid.len() - 1;

    let mut sum = 0;

    let mut op: Option<fn(i64, i64) -> i64> = None;
    let mut res = 0;

    let width = grid.iter().map(|r| r.len()).max().unwrap_or(0);

    for j in 0..width {
        if let Some(&op_char) = grid[last].get(j)
            && op_char != ' '
        {
            let (f, start) = get_op(op_char);
            op = Some(f);
            res = start;
        }

        let mut num: i64 = 0;
        for row in grid.iter().take(last) {
            if let Some(&ch) = row.get(j)
                && let Some(d) = ch.to_digit(10)
            {
                num = num * 10 + d as i64;
            }
        }

        if num == 0 {
            sum += res;
            res = 0;
            continue;
        }

        res = op.expect("operator must be defined")(res, num);
    }

    sum + res // + last num
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let input = read_input(file);

    assert_eq!(part_one(input.numbers, input.operators), 4277556);
    assert_eq!(part_two(file), 3263827);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let input = read_input(file);

    assert_eq!(part_one(input.numbers, input.operators), 5335495999141);
    assert_eq!(part_two(file), 10142723156431);
}
