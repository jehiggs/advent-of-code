use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const INPUT: &str = "./2024/day-2/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let part_1_start = Instant::now();
    let reader = BufReader::new(File::open(INPUT)?);
    let input = reader.lines().map_while(Result::ok);
    let part_1 = part_1(input);
    let part_1_end = Instant::now();
    println!("Part 1: Sum is {part_1}");
    println!(
        "Part 1 took {}s.",
        (part_1_end - part_1_start).as_secs_f64()
    );

    let part_2_start = Instant::now();
    let part_2_reader = BufReader::new(File::open(INPUT)?);
    let part_2_input = part_2_reader.lines().map_while(Result::ok);
    let part_2 = part_2(part_2_input);
    let part_2_end = Instant::now();
    println!("Part 2: Sum is {part_2}");
    println!(
        "Part 2 took {}s.",
        (part_2_end - part_2_start).as_secs_f64()
    );
    Ok(())
}

fn part_1<I: Iterator<Item = impl AsRef<str>>>(input: I) -> usize {
    input.fold(0, |acc, line| {
        let digit_iter = line
            .as_ref()
            .split(' ')
            .map(|digit| digit.parse::<usize>().expect("Should be a digit"))
            .pairs();

        let mut result = 1;
        let mut direction = None;
        for (a, b) in digit_iter {
            let (safe, increasing) = check_safe(a, b, direction);
            if direction.is_none() {
                direction = increasing;
            }
            if !safe {
                result = 0;
                break;
            }
        }

        acc + result
    })
}

fn part_2<I: Iterator<Item = impl AsRef<str>>>(input: I) -> usize {
    input.fold(0, |acc, line| {
        let digit_iter = line
            .as_ref()
            .split(' ')
            .map(|digit| digit.parse::<usize>().expect("Should be a digit"));
        let mut sequences = SubSequence::new(digit_iter);
        let safe = sequences.any(|sequence| {
            let seq_iter = sequence.into_iter().pairs();
            let mut direction = None;
            let mut is_safe = true;
            for (a, b) in seq_iter {
                let (safe, increasing) = check_safe(a, b, direction);
                if direction.is_none() {
                    direction = increasing;
                }
                if !safe {
                    is_safe = false;
                    break;
                }
            }
            is_safe
        });
        if safe { acc + 1 } else { acc }
    })
}

fn check_safe(a: usize, b: usize, increasing: Option<bool>) -> (bool, Option<bool>) {
    if a == b || a.abs_diff(b) > 3 {
        return (false, increasing);
    }
    match increasing {
        Some(true) if a > b => (false, increasing),
        Some(false) if a < b => (false, increasing),
        None => (true, Some(a < b)),
        _ => (true, increasing),
    }
}

struct PairIter<I> {
    iterator: I,
    last: Option<usize>,
}

impl<I: Iterator<Item = usize>> Iterator for PairIter<I> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.last.is_none() {
            self.last = self.iterator.next();
        }
        match (self.last, self.iterator.next()) {
            (Some(a), Some(b)) => {
                self.last = Some(b);
                Some((a, b))
            }
            _ => None,
        }
    }
}

trait PairIterable {
    fn pairs(self) -> PairIter<Self>
    where
        Self: Sized;
}

impl<I> PairIterable for I
where
    I: Iterator,
    I: Sized,
{
    fn pairs(self) -> PairIter<Self>
    where
        Self: Sized,
    {
        PairIter {
            iterator: self,
            last: None,
        }
    }
}

struct SubSequence {
    values: Vec<usize>,
    seq_num: usize,
}

impl Iterator for SubSequence {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seq_num >= self.values.len() {
            return None;
        }
        let values: Vec<_> = self
            .values
            .iter()
            .enumerate()
            .filter_map(|(i, value)| {
                if i == self.seq_num {
                    None
                } else {
                    Some(*value)
                }
            })
            .collect();
        self.seq_num += 1;
        Some(values)
    }
}

impl SubSequence {
    fn new<I: Iterator<Item = usize>>(iterator: I) -> Self {
        SubSequence {
            values: iterator.collect(),
            seq_num: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
6 1 2 3 4
1 2 3 4 8
8 4 7 6 5";

    #[test]
    fn check_part_1() {
        let result = part_1(SAMPLE.split('\n'));
        assert_eq!(2, result);
    }

    #[test]
    fn check_part_2() {
        let result = part_2(SAMPLE.split('\n'));
        assert_eq!(7, result);
    }
}
