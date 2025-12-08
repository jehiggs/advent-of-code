use aoc_lib::runner;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
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
    let vectors: Vec<_> = input
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
        .collect();
    vectors
        .iter()
        .map(|vector| {
            let closest = vectors
                .iter()
                .filter(|conn| vector != *conn)
                .min_by(|a, b| f64::total_cmp(&vector.diff(a), &vector.diff(b)))
                .expect("Should be an element");
            let distance = vector.diff(closest);
            (closest.x * vector.x, distance)
        })
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(x, _)| x)
        .expect("Should be an element")
}

fn part_1_parameterized(input: &str, num_connections: usize) -> usize {
    let vectors: Vec<_> = input
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
        .collect();
    let mut connections: Vec<(usize, usize, f64)> = Vec::with_capacity(num_connections + 1);
    for (i, vector) in vectors.iter().enumerate() {
        for (j, conn) in vectors[i + 1..].iter().enumerate() {
            let len = vector.diff(conn);
            connections.push((i, j + i + 1, len));
            connections.sort_unstable_by(|(_, _, a), (_, _, b)| a.total_cmp(b));
            if connections.len() > num_connections {
                connections.pop();
            }
        }
    }
    let mut graphs: HashMap<usize, Vec<usize>> = HashMap::with_capacity(num_connections);
    for conn in connections {
        if let Some(val) = graphs.get_mut(&conn.0) {
            val.push(conn.1);
        } else {
            graphs.insert(conn.0, Vec::from([conn.1]));
        }

        if let Some(val) = graphs.get_mut(&conn.1) {
            val.push(conn.0);
        } else {
            graphs.insert(conn.1, Vec::from([conn.0]));
        }
    }

    let mut circuit_sizes = Vec::new();
    let mut visited = HashSet::new();
    for key in graphs.keys() {
        if !visited.contains(key) {
            let mut circuit_size = 0;
            let mut queue = Vec::from([*key]);
            visited.insert(key);
            while let Some(k) = queue.pop() {
                circuit_size += 1;
                if let Some(val) = graphs.get(&k) {
                    for next_k in val {
                        if !visited.contains(next_k) {
                            queue.push(*next_k);
                            visited.insert(next_k);
                        }
                    }
                }
            }
            circuit_sizes.push(circuit_size);
        }
    }
    circuit_sizes.sort_unstable_by_key(|num| Reverse(*num));
    circuit_sizes.iter().take(3).product()
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

    fn diff(&self, other: &Vec3) -> f64 {
        let total = self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2);
        (total as f64).sqrt()
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
