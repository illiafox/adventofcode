use std::str::Lines;

enum Direction {
    Left(i32),
    Right(i32),
}

struct Input {
    directions: Vec<Direction>,
}

fn read_input(lines: Lines) -> Input {
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

    Input { directions }
}

fn normalize(current: i32) -> i32 {
    current.rem_euclid(100)
}

fn part_one(input: &Input) -> i32 {
    let mut current = 50;

    let mut total_zeroes = 0;
    for d in input.directions.iter() {
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

fn part_two(input: &Input) -> i32 {
    let mut current = 50;

    let mut total_zero_clicks = 0;

    for d in input.directions.iter() {
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
fn test() {
    let file = include_str!("input.txt");
    let input = read_input(file.lines());

    let total_zeroes = part_one(&input);
    println!("part 1: total zeroes: {total_zeroes}");
    assert_eq!(total_zeroes, 1158);

    let part_two = part_two(&input);
    println!("part 2: total any zeroes clicks: {part_two}");
    assert_eq!(part_two, 6860);
}
