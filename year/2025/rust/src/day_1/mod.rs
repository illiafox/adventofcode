use std::str::Lines;

enum Direction {
    Left(i32),
    Right(i32),
}

fn parse_directions(lines: Lines) -> Vec<Direction> {
    let mut directions = Vec::new();

    for line in lines {
        let (d, angle_str) = line.split_at(1);

        let angle: i32 = angle_str.parse().unwrap();

        let direction = match d {
            "L" => Direction::Left(angle),
            "R" => Direction::Right(angle),
            _ => panic!("invalid input"),
        };

        directions.push(direction);
    }

    directions
}

fn normalize(current: i32) -> i32 {
    current.rem_euclid(100)
}

fn part_one(directions: &[Direction]) -> i32 {
    let mut current = 50;

    let mut total_zeroes = 0;
    for d in directions.iter() {
        match d {
            Direction::Left(angle) => current -= angle,
            Direction::Right(angle) => current += angle,
        }

        current = normalize(current);
        if current == 0 {
            total_zeroes += 1;
        }
    }

    total_zeroes
}

fn part_two(directions: &[Direction]) -> i32 {
    let mut current = 50;

    let mut total_zero_clicks = 0;

    for d in directions.iter() {
        let start = current;

        match d {
            Direction::Left(angle) => {
                let step = angle % 100;
                current = normalize(start - step);

                total_zero_clicks += angle / 100;
                if start > 0 && step >= start {
                    total_zero_clicks += 1;
                }
            }

            Direction::Right(angle) => {
                let step = angle % 100;
                current = normalize(start + step);

                total_zero_clicks += angle / 100;
                if start + step > 99 {
                    total_zero_clicks += 1;
                }
            }
        }
    }

    total_zero_clicks
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let dirs = parse_directions(file.lines());

    assert_eq!(part_one(&dirs), 3);
    assert_eq!(part_two(&dirs), 6);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let directions = parse_directions(file.lines());

    assert_eq!(part_one(&directions), 1158);
    assert_eq!(part_two(&directions), 6860);
}
