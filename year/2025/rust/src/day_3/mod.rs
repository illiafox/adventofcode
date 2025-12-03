struct Input {
    banks: Vec<Vec<i8>>,
}

fn read_input(s: &str) -> Input {
    let banks = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    Input { banks }
}

fn get_2_joltage(bank: &[i8]) -> i8 {
    let mut sorted = bank.to_vec();
    sorted.sort_unstable();

    for &first in sorted.iter().rev().take(2) {
        let idx = bank.iter().position(|&x| x == first).unwrap();

        if idx + 1 < bank.len()
            && let Some(&second) = bank[idx + 1..].iter().max()
        {
            return first * 10 + second;
        }
    }

    panic!("no valid joltage pair found");
}

fn part_one(input: &Input) -> i64 {
    input.banks.iter().map(|b| get_2_joltage(b) as i64).sum()
}

fn find_next_joltage_digit(bank: &[i8], digits_left: usize) -> (i8, usize) {
    let search_region = &bank[0..=bank.len() - digits_left];
    let m = *search_region.iter().max().unwrap();

    let idx = search_region.iter().position(|&x| x == m).unwrap();

    (m, idx)
}

fn get_12_joltage(bank: &[i8]) -> i64 {
    let mut joltage: i64 = 0;
    let mut bank = bank;

    for digits_left in (1..=12).rev() {
        let (digit, idx) = find_next_joltage_digit(bank, digits_left);
        joltage = joltage * 10 + digit as i64;
        bank = &bank[idx + 1..]
    }

    joltage
}

fn part_two(input: &Input) -> i64 {
    input.banks.iter().map(|b| get_12_joltage(b)).sum()
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let input = read_input(file);

    let part_one = part_one(&input);
    assert_eq!(part_one, 357);

    let part_two = part_two(&input);
    assert_eq!(part_two, 3121910778619);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let input = read_input(file);

    let part_one = part_one(&input);
    assert_eq!(part_one, 17031);

    let part_two = part_two(&input);
    assert_eq!(part_two, 168575096286051);
}
