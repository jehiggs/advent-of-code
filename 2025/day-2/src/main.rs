use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time;

// Solves 2025 day 2 by generating candidate invalid IDs and summing them.
fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("./2025/day-2/input.txt")?);
    for line in reader.lines().map_while(Result::ok) {
        let part_1_start = time::Instant::now();
        let result = line
            .split(',')
            .try_fold(0, |acc, value| part_1(value).map(|result| acc + result))?;
        let part_1_end = time::Instant::now();
        println!("Part 1: The sum of invalid IDs is {result}");
        println!("Part 1 took {}s", (part_1_end - part_1_start).as_secs_f64());
    }

    let part_2_reader = BufReader::new(File::open("./2025/day-2/input.txt")?);
    for line in part_2_reader.lines().map_while(Result::ok) {
        let part_2_start = time::Instant::now();
        let result = line
            .split(',')
            .try_fold(0, |acc, value| part_2(value).map(|result| acc + result))?;
        let part_2_end = time::Instant::now();
        println!("Part 2: The sum of invalid IDs is {result}");
        println!("Part 2 took {}s", (part_2_end - part_2_start).as_secs_f64());
    }
    Ok(())
}

#[allow(clippy::cast_possible_truncation)]
fn part_1(string: &str) -> Result<usize, Box<dyn Error>> {
    let (lower_str, upper_str) = string
        .split_once('-')
        .ok_or_else(|| error("Range was not provided in a valid format"))?;
    let lower = if lower_str.len() % 2 == 0 {
        let mid = lower_str.len() / 2;
        let (first, second) = lower_str.split_at(mid);
        let bound = first.parse::<usize>()?;
        let part = second.parse::<usize>()?;
        if bound < part { bound + 1 } else { bound }
    } else {
        let power = lower_str.len() / 2;
        10usize.pow(power as u32)
    };

    let upper = if upper_str.len() % 2 == 0 {
        let mid = upper_str.len() / 2;
        let (first, second) = upper_str.split_at(mid);
        let bound = first.parse::<usize>()?;
        let part = second.parse::<usize>()?;
        if bound > part { bound } else { bound + 1 }
    } else {
        let power = upper_str.len() / 2;
        10usize.pow(power as u32)
    };

    if upper <= lower {
        Ok(0)
    } else {
        Ok((lower..upper).fold(0, |acc, next| {
            acc + 10usize.pow(next.ilog10() + 1) * next + next
        }))
    }
}

#[allow(clippy::cast_possible_truncation)]
fn part_2(range: &str) -> Result<usize, Box<dyn Error>> {
    let (lower_str, upper_str) = range
        .split_once('-')
        .ok_or_else(|| error("Range was not provided in a valid format"))?;
    let min_len = cmp::max(lower_str.len(), 2);
    let max_len = upper_str.len();
    if max_len == 1 {
        return Ok(0);
    }
    let mut result = 0;
    let minimum = lower_str.parse::<usize>()?;
    let maximum = upper_str.parse::<usize>()?;
    let mut ids = HashSet::new();
    // Generate numbers of lengths between min and max
    for length in min_len..=max_len {
        // Generate sequences to repeat
        for sequence_length in (1..=length / 2).filter(|s| length % s == 0) {
            let min_seq = 10usize.pow(sequence_length as u32 - 1);
            let max_seq = 10usize.pow(sequence_length as u32) - 1;
            // Generate each value that fits.
            for value in min_seq..=max_seq {
                let mut invalid_id = 0;
                for power in (0..length).step_by(sequence_length) {
                    invalid_id += 10usize.pow(power as u32) * value;
                }
                if !ids.contains(&invalid_id) && minimum <= invalid_id && invalid_id <= maximum {
                    result += invalid_id;
                    ids.insert(invalid_id);
                }
            }
        }
    }
    Ok(result)
}

#[derive(Debug)]
struct RangeError {
    reason: &'static str,
}

impl Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for RangeError {}

fn error(msg: &'static str) -> Box<dyn Error> {
    Box::new(RangeError { reason: msg })
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    use crate::part_2;

    #[test]
    fn verify_part_1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = input
            .split(',')
            .fold(0, |acc, value| part_1(value).expect("Input is valid") + acc);
        assert_eq!(1_227_775_554, result);
    }

    #[test]
    fn verify_part_2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = input
            .split(',')
            .fold(0, |acc, value| part_2(value).expect("Input is valid") + acc);
        assert_eq!(4_174_379_265, result);
    }
}
