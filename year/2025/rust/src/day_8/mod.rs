use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3 {
    fn distance(&self, other: &Point3) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

fn parse_grid(s: &str) -> Vec<Point3> {
    s.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut it = line.split(',');
            Point3 {
                x: it.next().unwrap().parse().unwrap(),
                y: it.next().unwrap().parse().unwrap(),
                z: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    total_sets: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            total_sets: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.total_sets -= 1;
        true
    }

    fn component_sizes(&mut self) -> Vec<i64> {
        let mut counts: HashMap<usize, i64> = HashMap::new();
        for i in 0..self.parent.len() {
            let r = self.find(i);
            *counts.entry(r).or_insert(0) += 1;
        }
        counts.into_values().collect()
    }
}

fn build_edges(boxes: &[Point3]) -> Vec<(i64, usize, usize)> {
    let n = boxes.len();
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((boxes[i].distance(&boxes[j]), i, j));
        }
    }

    edges.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    edges
}

fn part_one(boxes: &[Point3], k: usize) -> i64 {
    let n = boxes.len();
    let edges = build_edges(boxes);

    let mut dsu = Dsu::new(n);
    for &(_, i, j) in edges.iter().take(k.min(edges.len())) {
        dsu.union(i, j);
    }

    let mut sizes = dsu.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes.iter().take(3).product()
}

fn part_two(boxes: &[Point3]) -> i64 {
    let n = boxes.len();
    let edges = build_edges(boxes);

    let mut dsu = Dsu::new(n);
    let mut last_merge = (0usize, 0usize);

    for &(_, i, j) in &edges {
        if dsu.union(i, j) {
            last_merge = (i, j);
            if dsu.total_sets == 1 {
                break;
            }
        }
    }

    let (i, j) = last_merge;
    boxes[i].x as i64 * boxes[j].x as i64
}

#[test]
fn test_example() {
    let file = include_str!("example_input.txt");
    let boxes = parse_grid(file);

    assert_eq!(part_one(&boxes, 10), 40);
    assert_eq!(part_two(&boxes), 25272);
}

#[test]
fn test_custom_input() {
    let file = include_str!("input.txt");
    let boxes = parse_grid(file);

    assert_eq!(part_one(&boxes, 1000), 75680);
    assert_eq!(part_two(&boxes), 8995844880);
}
