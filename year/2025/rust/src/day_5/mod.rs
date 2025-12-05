struct Input {
    ranges: Vec<(usize, usize)>,
    check_ids: Vec<usize>,
}

fn read_input(s: &str) -> Input {
    let (ranges_block, ids_block) = s.split_once("\n\n").expect("expected split");

    let ranges = ranges_block
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').expect("range missing '-'");
            let start = a.trim().parse().unwrap();
            let end = b.trim().parse().unwrap();
            (start, end)
        })
        .collect::<Vec<(usize, usize)>>();

    let check_ids = ids_block
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    Input { ranges, check_ids }
}

fn merge_ranges(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    let mut ranges = ranges.to_vec();
    ranges.sort_by_key(|r| r.0);

    let mut merged = Vec::new();

    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for &(start, end) in &ranges[1..] {
        if start <= current_end {
            current_end = current_end.max(end);
        } else {
            merged.push((current_start, current_end));
            current_start = start;
            current_end = end;
        }
    }

    merged.push((current_start, current_end));

    merged
}

fn search_binary(ranges: &[(usize, usize)], id: usize) -> bool {
    let idx = ranges.partition_point(|&(start, _)| start <= id);
    if idx == 0 {
        return false;
    }
    let (start, end) = ranges[idx - 1];
    id >= start && id <= end
}

fn part_one(input: &Input) -> i64 {
    let merged = merge_ranges(&input.ranges);

    let mut fresh_count = 0;

    for &id in input.check_ids.iter() {
        if search_binary(&merged, id) {
            fresh_count += 1
        }
    }

    fresh_count
}

fn part_two(input: &Input) -> i64 {
    let mut fresh_count = 0;

    let merged = merge_ranges(&input.ranges);
    for &(start, end) in merged.iter() {
        fresh_count += end - start + 1
    }

    fresh_count as i64
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let map = read_input(file);

    assert_eq!(part_one(&map), 3);
    assert_eq!(part_two(&map), 14);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let map = read_input(file);

    assert_eq!(part_one(&map), 674);
    assert_eq!(part_two(&map), 352509891817881);
}
