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
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let operators = op_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<char>>();

    Input { numbers, operators }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

    lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn part_one(numbers: Vec<Vec<i64>>, operators: Vec<char>) -> i64 {
    let mut sum = 0;

    for j in 0..operators.len() {
        let op = match operators[j] {
            '+' => |a, b| a + b,
            '*' => |a, b| a * b,
            _ => panic!("unsupported char"),
        };

        let mut res = numbers[0][j];
        for i in 1..numbers.len() {
            res = op(res, numbers[i][j])
        }

        sum += res
    }

    sum
}

fn part_two(file: &str) -> i64 {
    let mut sum = 0;

    let grid = parse_grid(file);

    let mut res = 0;
    let mut op: fn(i64, i64) -> i64 = |_, _| panic!("wrong op!");

    for j in 0..grid[0].len() {
        if grid[grid.len() - 1].len() > j {
            let op_char = grid[grid.len() - 1][j];
            if op_char != ' ' {
                op = match op_char {
                    '+' => |a, b| a + b,
                    '*' => |a, b| a * b,
                    _ => panic!("wrong op!"),
                };
                res = match op_char {
                    '+' => 0,
                    '*' => 1,
                    _ => panic!("wrong op!"),
                }
            }
        }

        let mut num = 0;
        for row in grid.iter().take(grid.len() - 1) {
            if j < row.len()
                && let Some(d) = row[j].to_digit(10)
            {
                num = num * 10 + d
            }
        }

        if num == 0 {
            sum += res;
            res = 0;
            continue;
        }

        res = op(res, num as i64);
    }

    sum += res;

    sum
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
