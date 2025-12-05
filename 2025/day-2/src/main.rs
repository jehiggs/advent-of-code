use aoc_lib::runner;
use std::cmp;
use std::collections::HashSet;
use std::error::Error;

const INPUT: &str = "./2025/day-2/input.txt";

// Solves 2025 day 2 by generating candidate invalid IDs and summing them.
fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, |input| {
        input
            .trim()
            .split(',')
            .fold(0, |acc, value| part_1(value) + acc)
    })?;

    runner::run("Part 2", INPUT, |input| {
        input
            .trim()
            .split(',')
            .fold(0, |acc, value| part_2(value) + acc)
    })?;
    Ok(())
}

#[allow(clippy::cast_possible_truncation)]
fn part_1(string: &str) -> usize {
    let (lower_str, upper_str) = string.split_once('-').expect("Should be separated by '-'");
    let lower = if lower_str.len() % 2 == 0 {
        let mid = lower_str.len() / 2;
        let (first, second) = lower_str.split_at(mid);
        let bound = first.parse::<usize>().expect("Should be an integer");
        let part = second.parse::<usize>().expect("Should be an integer");
        if bound < part { bound + 1 } else { bound }
    } else {
        let power = lower_str.len() / 2;
        10usize.pow(power as u32)
    };

    let upper = if upper_str.len() % 2 == 0 {
        let mid = upper_str.len() / 2;
        let (first, second) = upper_str.split_at(mid);
        let bound = first.parse::<usize>().expect("Should be an integer");
        let part = second.parse::<usize>().expect("Should be an integer");
        if bound > part { bound } else { bound + 1 }
    } else {
        let power = upper_str.len() / 2;
        10usize.pow(power as u32)
    };

    if upper <= lower {
        0
    } else {
        (lower..upper).fold(0, |acc, next| {
            acc + 10usize.pow(next.ilog10() + 1) * next + next
        })
    }
}

#[allow(clippy::cast_possible_truncation)]
fn part_2(range: &str) -> usize {
    let (lower_str, upper_str) = range.split_once('-').expect("Should be separated by '-'");
    let min_len = cmp::max(lower_str.len(), 2);
    let max_len = upper_str.len();
    if max_len == 1 {
        return 0;
    }
    let mut result = 0;
    let minimum = lower_str.parse::<usize>().expect("Should be an integer");
    let maximum = upper_str.parse::<usize>().expect("Should be an integer");
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
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn verify_part_1() {
        let result = SAMPLE.split(',').fold(0, |acc, value| part_1(value) + acc);
        assert_eq!(1_227_775_554, result);
    }

    #[test]
    fn verify_part_2() {
        let result = SAMPLE.split(',').fold(0, |acc, value| part_2(value) + acc);
        assert_eq!(4_174_379_265, result);
    }
}
