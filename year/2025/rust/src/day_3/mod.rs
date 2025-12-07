fn parse_banks(s: &str) -> Vec<Vec<i8>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}

fn get_joltage(mut bank: &[i8], digits_count: usize) -> i64 {
    assert!(bank.len() >= digits_count);

    let mut joltage = 0;

    for digits_left in (1..=digits_count).rev() {
        let search_region = &bank[..=bank.len() - digits_left];

        let mut digit = -1;
        let mut idx = 0;

        for (i, &d) in search_region.iter().enumerate() {
            if d > digit {
                digit = d;
                idx = i;
            }
        }

        joltage = joltage * 10 + digit as i64;
        bank = &bank[idx + 1..];
    }

    joltage
}

fn part_one(banks: &[Vec<i8>]) -> i64 {
    banks.iter().map(|b| get_joltage(b, 2)).sum()
}

fn part_two(banks: &[Vec<i8>]) -> i64 {
    banks.iter().map(|b| get_joltage(b, 12)).sum()
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let banks = parse_banks(file);

    assert_eq!(part_one(&banks), 357);
    assert_eq!(part_one_old(&banks), 357);
    assert_eq!(part_two(&banks), 3121910778619);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let banks = parse_banks(file);

    assert_eq!(part_one(&banks), 17031);
    assert_eq!(part_one_old(&banks), 17031);
    assert_eq!(part_two(&banks), 168575096286051);
}

// old version
fn part_one_old(banks: &[Vec<i8>]) -> i64 {
    banks.iter().map(|d| get_2_joltage(d)).sum()
}

fn get_2_joltage(bank: &[i8]) -> i64 {
    assert!(bank.len() >= 2);

    let mut sorted = bank.to_vec();
    sorted.sort_unstable();

    for &first in sorted.iter().rev().take(2) {
        let idx = bank.iter().position(|&x| x == first).unwrap();

        if idx + 1 < bank.len()
            && let Some(&second) = bank[idx + 1..].iter().max()
        {
            return (first * 10 + second) as i64;
        }
    }

    panic!("no valid joltage pair found");
}
