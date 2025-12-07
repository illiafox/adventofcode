fn parse_ranges(s: &str) -> Vec<(i64, i64)> {
    let mut ranges = Vec::new();

    for line in s.split(',') {
        let (start_str, end_str) = line.split_once('-').unwrap();

        let start: i64 = start_str.parse().unwrap();
        let end: i64 = end_str.parse().unwrap();

        ranges.push((start, end));
    }

    ranges
}

fn digit_count(mut n: i64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut c = 0;
    while n > 0 {
        n /= 10;
        c += 1;
    }
    c
}

pub fn has_repeated_half(n: i64) -> bool {
    if n < 0 {
        return false;
    }

    let len = digit_count(n);
    if !len.is_multiple_of(2) {
        return false;
    }

    let pow = 10_i64.pow((len / 2) as u32);

    n % pow == n / pow
}

// https://leetcode.com/problems/repeated-substring-pattern
pub fn repeats_at_least_twice(n: i64) -> bool {
    let s = n.to_string();

    let mut doubled = String::with_capacity(s.len() * 2);
    doubled.push_str(&s);
    doubled.push_str(&s);

    doubled[1..doubled.len() - 1].contains(&s)
}

fn part_one(ranges: &[(i64, i64)]) -> i64 {
    let mut sum = 0;

    for &(start, end) in ranges.iter() {
        for n in start..=end {
            if has_repeated_half(n) {
                sum += n
            }
        }
    }

    sum
}

fn part_two(ranges: &[(i64, i64)]) -> i64 {
    let mut sum = 0;

    for &(start, end) in ranges.iter() {
        for n in start..=end {
            if repeats_at_least_twice(n) {
                sum += n
            }
        }
    }

    sum
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let ranges = parse_ranges(file);

    assert_eq!(part_one(&ranges), 1227775554);
    assert_eq!(part_two(&ranges), 4174379265);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let ranges = parse_ranges(file);

    assert_eq!(part_one(&ranges), 28846518423);
    assert_eq!(part_two(&ranges), 31578210022);
}
