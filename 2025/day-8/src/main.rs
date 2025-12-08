use aoc_lib::runner;
use std::cmp::Reverse;
use std::error::Error;

const INPUT: &str = "./2025/day-8/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    part_1_parameterized(input, 1000)
}

fn part_2(input: &str) -> usize {
    let vectors = parse_vectors(input);
    vectors
        .iter()
        .map(|vector| {
            let closest = vectors
                .iter()
                .filter(|conn| vector != *conn)
                .min_by_key(|v| vector.diff(v))
                .expect("Should be an element");
            let distance = vector.diff(closest);
            (closest.x * vector.x, distance)
        })
        .max_by_key(|(_, a)| *a)
        .map(|(x, _)| x)
        .expect("Should be an element")
}

fn part_1_parameterized(input: &str, num_connections: usize) -> usize {
    let vectors = parse_vectors(input);
    let mut connections: Vec<_> = vectors
        .iter()
        .enumerate()
        .flat_map(|(i, vector)| {
            vectors[i + 1..]
                .iter()
                .enumerate()
                .map(move |(j, conn)| (i, j + i + 1, vector.diff(conn)))
        })
        .collect();
    connections.sort_unstable_by_key(|(_, _, a)| *a);
    let mut set = DisjointSet::new(vectors.len());
    for connection in connections.iter().take(num_connections) {
        set.union(connection.0, connection.1);
    }
    let mut sizes = set.sizes();
    sizes.sort_unstable_by_key(|num| Reverse(*num));
    sizes.iter().take(3).product()
}

fn parse_vectors(input: &str) -> Vec<Vec3> {
    input
        .split('\n')
        .map(|line| {
            let (x, rest) = line.split_once(',').expect("Comma separated digits");
            let (y, z) = rest.split_once(',').expect("Comma separated digits");
            Vec3::new(
                x.parse().expect("Number"),
                y.parse().expect("Number"),
                z.parse().expect("Number"),
            )
        })
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: usize,
    y: usize,
    z: usize,
}

#[allow(clippy::cast_precision_loss)]
impl Vec3 {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Vec3 { x, y, z }
    }

    fn diff(&self, other: &Vec3) -> usize {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

#[derive(Debug)]
struct DisjointSet {
    parents: Vec<(usize, usize)>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        DisjointSet {
            parents: (0..size).map(|i| (i, 1)).collect(),
        }
    }

    fn find(&mut self, item: usize) -> usize {
        if self.parents[item].0 != item {
            self.parents[item].0 = self.find(self.parents[item].0);
            self.parents[item].1 = 0;
        }
        self.parents[item].0
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (root_a, root_b) = (self.find(a), self.find(b));
        if root_a == root_b {
            return false;
        }

        let (new_root, merged) = if self.parents[root_a].1 > self.parents[root_b].1 {
            (root_a, root_b)
        } else {
            (root_b, root_a)
        };

        self.parents[merged].0 = self.parents[new_root].0;
        self.parents[new_root].1 += self.parents[merged].1;
        true
    }

    fn sizes(&self) -> Vec<usize> {
        self.parents
            .iter()
            .enumerate()
            .filter_map(|(i, (root, size))| (i == *root && *size != 0).then_some(*size))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn verify_part_1() {
        let result = part_1_parameterized(SAMPLE, 10);
        assert_eq!(40, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(25272, result);
    }
}
