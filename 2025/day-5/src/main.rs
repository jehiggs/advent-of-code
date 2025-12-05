use aoc_lib::runner;
use std::cmp::max;
use std::error::Error;

const INPUT: &str = "./2025/day-5/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    let (ranges, values) = input
        .split_once("\n\n")
        .expect("Should be a double new line to split input");
    let merged = merged_ranges(ranges);
    values
        .split('\n')
        .map(|line| line.parse().expect("Integer"))
        .filter(|value| search(*value, &merged))
        .count()
}

fn part_2(input: &str) -> usize {
    let (range_str, _) = input
        .split_once("\n\n")
        .expect("Should be a double new line to split input");
    merged_ranges(range_str)
        .iter()
        .fold(0, |acc, (min, max)| acc + max - min + 1)
}

fn search(value: usize, ranges: &[(usize, usize)]) -> bool {
    let (mut left, mut right) = (0, ranges.len());
    while right - left > 0 {
        let mid = left.midpoint(right);
        let range = ranges[mid];
        if value >= range.0 && value <= range.1 {
            return true;
        } else if value < range.0 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    false
}

fn merged_ranges(ranges: &str) -> Vec<(usize, usize)> {
    let mut initial: Vec<_> = ranges
        .split('\n')
        .map(|line| {
            line.split_once('-')
                .map(|(min_str, max_str)| {
                    (
                        min_str.parse().expect("Integer"),
                        max_str.parse().expect("Integer"),
                    )
                })
                .expect("- delimiter expected")
        })
        .collect();
    initial.sort_unstable();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for range in initial {
        if let Some(last) = merged.last_mut()
            && last.1 >= range.0
        {
            last.1 = max(last.1, range.1);
        } else {
            merged.push(range);
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(3, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(14, result);
    }
}
