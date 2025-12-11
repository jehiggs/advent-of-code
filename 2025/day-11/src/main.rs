use aoc_lib::runner;
use std::collections::HashMap;
use std::error::Error;

const INPUT: &str = "./2025/day-11/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    let graph = parse(input);
    route_count(&graph, "you")
}

fn route_count(graph: &HashMap<String, Vec<String>>, key: &str) -> usize {
    let mut count = 0;
    let next = &graph[key];
    for next_key in next {
        if next_key == "out" {
            count += 1;
        } else {
            count += route_count(graph, next_key);
        }
    }
    count
}

fn part_2(input: &str) -> usize {
    let graph = parse(input);
    let mut visited = HashMap::new();
    route_with_stops_count(&graph, &mut visited, "svr").both
}

fn route_with_stops_count(
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashMap<String, Counts>,
    key: &str,
) -> Counts {
    let mut counts = Counts::new();
    let bitmask = match key {
        "fft" => 0b01,
        "dac" => 0b10,
        _ => 0b00,
    };
    let next = &graph[key];
    for next_key in next {
        if next_key == "out" {
            counts.increment(bitmask);
        } else if let Some(count) = visited.get(next_key) {
            counts.add(count, bitmask);
        } else {
            let count = route_with_stops_count(graph, visited, next_key);
            counts.add(&count, bitmask);
        }
    }
    visited.insert(key.to_owned(), counts);
    counts
}

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    input
        .split('\n')
        .map(|line| {
            let (key, value) = line.split_once(':').expect("Should be a ':'.");
            let values = value.trim().split(' ').map(str::to_owned).collect();
            (key.to_owned(), values)
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Counts {
    neither: usize,
    dac: usize,
    fft: usize,
    both: usize,
}

impl Counts {
    fn new() -> Self {
        Counts {
            neither: 0,
            dac: 0,
            fft: 0,
            both: 0,
        }
    }

    fn increment(&mut self, bitmask: u8) {
        match bitmask {
            0b00 => self.neither += 1,
            0b01 => self.fft += 1,
            0b10 => self.dac += 1,
            0b11 => self.both += 1,
            _ => {}
        }
    }

    fn add(&mut self, other: &Counts, bitmask: u8) {
        match bitmask {
            0b00 => {
                self.neither += other.neither;
                self.fft += other.fft;
                self.dac += other.dac;
                self.both += other.both;
            }
            0b01 => {
                self.fft += other.neither + other.fft;
                self.both += other.both + other.dac;
            }
            0b10 => {
                self.dac += other.neither + other.dac;
                self.both += other.both + other.fft;
            }
            0b11 => {
                self.both += other.neither + other.dac + other.fft + other.both;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const SAMPLE_PART_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(5, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE_PART_2);
        assert_eq!(2, result);
    }
}
