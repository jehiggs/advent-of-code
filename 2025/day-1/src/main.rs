use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let part_1 = solve(Counter::apply_rotation_1)?;
    println!("Part 1 result: {part_1}");
    let part_2 = solve(Counter::apply_rotation_2)?;
    println!("Part 2 result: {part_2}");
    Ok(())
}

fn solve(rotation_func: impl Fn(&mut Counter, &Rotation)) -> Result<usize, Box<dyn Error>> {
    let reader = io::BufReader::new(fs::File::open("./2025/day-1/input/1.txt")?);
    let result = reader
        .lines()
        .map(|line| match line {
            Ok(string) => Rotation::from(&string),
            Err(err) => Err(Box::new(err) as Box<dyn Error>),
        })
        .try_fold(Counter::new(), |mut value, rotation| {
            rotation.map(|rot| {
                rotation_func(&mut value, &rot);
                value
            })
        })?;
    Ok(result.zero_count)
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
