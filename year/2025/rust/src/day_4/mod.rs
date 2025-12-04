struct Input {
    map: Vec<Vec<Tile>>,
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Roll,
}

fn read_input(s: &str) -> Vec<Vec<Tile>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '@' => Tile::Roll,
                    _ => panic!("wrong input char {c}"),
                })
                .collect()
        })
        .collect()
}

fn remove_rolls(map: &[Vec<Tile>]) -> (i64, Vec<(usize, usize)>) {
    let mut rolls_removed = 0;
    let mut points = Vec::new();

    let height = map.len();

    for i in 0..height {
        let width = map[i].len();
        for j in 0..width {
            let elem = map[i][j];

            if let Tile::Empty = elem {
                continue;
            }

            let directions = [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (-1, -1),
                (-1, 1),
                (1, -1),
            ];

            let mut rolls = 0;

            for (dx, dy) in directions {
                let new_i = i as i32 + dy;
                let new_j = j as i32 + dx;

                if let Some(row) = map.get(new_i as usize)
                    && let Some(Tile::Roll) = row.get(new_j as usize)
                {
                    rolls += 1;
                }
            }

            if rolls < 4 {
                rolls_removed += 1;
                points.push((i, j));
            }
        }
    }

    (rolls_removed, points)
}

fn part_one(map: &[Vec<Tile>]) -> i64 {
    let (rolls_removed, _) = remove_rolls(map);

    rolls_removed
}

fn part_two(map: &[Vec<Tile>]) -> i64 {
    let mut map: Vec<Vec<Tile>> = map.to_vec();

    let mut removed_total = 0;

    loop {
        let (removed, rolls) = remove_rolls(&map);
        if removed == 0 {
            return removed_total;
        }

        removed_total += removed;

        for (i, j) in rolls {
            map[i][j] = Tile::Empty
        }
    }
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let map = read_input(file);

    assert_eq!(part_one(&map), 13);
    assert_eq!(part_two(&map), 43);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let map = read_input(file);

    assert_eq!(part_one(&map), 1569);
    assert_eq!(part_two(&map), 9280);
}
