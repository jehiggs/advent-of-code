use aoc_lib::runner;
use std::error::Error;

const INPUT: &str = "";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(0, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(0, result);
    }
}
