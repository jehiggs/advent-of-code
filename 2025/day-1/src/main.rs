use aoc_lib::runner;
use std::error::Error;
use std::fmt::Display;

const INPUT: &str = "./2025/day-1/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| Rotation::from(line))
        .fold(Counter::new(), |mut counter, rotation| {
            if let Ok(r) = rotation {
                counter.apply_rotation_1(&r);
            }
            counter
        })
        .zero_count
}

fn part_2(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| Rotation::from(line))
        .fold(Counter::new(), |mut counter, rotation| {
            if let Ok(r) = rotation {
                counter.apply_rotation_2(&r);
            }
            counter
        })
        .zero_count
}

#[derive(Debug)]
enum Rotation {
    Left(usize),
    Right(usize),
}

impl Rotation {
    fn from(string: &str) -> Result<Self, Box<dyn Error>> {
        let direction = string
            .chars()
            .nth(0)
            .ok_or_else(|| error("No first character to parse"))?;
        let amount: usize = string[1..].parse()?;
        match direction {
            'L' => Ok(Self::Left(amount)),
            'R' => Ok(Self::Right(amount)),
            _ => Err(error("Character must be either L or R")),
        }
    }
}

#[derive(Debug)]
struct Counter {
    position: usize,
    zero_count: usize,
}

impl Counter {
    fn new() -> Self {
        Counter {
            position: 50,
            zero_count: 0,
        }
    }

    fn apply_rotation_1(&mut self, rotation: &Rotation) {
        let new = match rotation {
            Rotation::Left(amount) => {
                let difference = amount % 100;
                (100 + self.position - difference) % 100
            }
            Rotation::Right(amount) => (self.position + amount) % 100,
        };
        if new == 0 {
            self.zero_count += 1;
        }
        self.position = new;
    }

    fn apply_rotation_2(&mut self, rotation: &Rotation) {
        let new = match rotation {
            Rotation::Left(amount) => {
                self.zero_count += amount / 100;
                let difference = amount % 100;
                let new = (100 + self.position - difference) % 100;
                if (new > self.position && self.position != 0) || new == 0 {
                    self.zero_count += 1;
                }
                new
            }
            Rotation::Right(amount) => {
                self.zero_count += amount / 100;
                let new = (self.position + amount) % 100;
                if new < self.position {
                    self.zero_count += 1;
                }
                new
            }
        };
        self.position = new;
    }
}

#[derive(Debug)]
struct RotationError {
    reason: &'static str,
}

impl Display for RotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for RotationError {}

fn error(msg: &'static str) -> Box<dyn Error> {
    Box::new(RotationError { reason: msg })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(3, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(6, result);
    }
}
