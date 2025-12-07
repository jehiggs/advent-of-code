use aoc_lib::runner;
use std::error::Error;

const INPUT: &str = "./2025/day-7/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    let mut grid = input.split('\n').map(|row| Vec::from(row.as_bytes()));
    let mut counter: Vec<_> = grid
        .next()
        .map(|row| row.iter().map(|b| usize::from(*b == b'S')).collect())
        .expect("Should be a first row");
    let mut total = 0;
    for row in grid {
        for (index, elem) in row.iter().enumerate() {
            let current_count = counter[index];
            if current_count > 0 && *elem == b'^' {
                if index > 0 {
                    counter[index - 1] = 1;
                }
                if index < row.len() - 1 {
                    counter[index + 1] = 1;
                }
                counter[index] = 0;
                total += 1;
            }
        }
    }
    total
}

fn part_2(input: &str) -> usize {
    let mut grid = input.split('\n').map(|row| Vec::from(row.as_bytes()));
    let mut counter: Vec<_> = grid
        .next()
        .map(|row| row.iter().map(|b| usize::from(*b == b'S')).collect())
        .expect("Should be a first row");
    for row in grid {
        for (index, elem) in row.iter().enumerate() {
            let current_count = counter[index];
            if current_count > 0 && *elem == b'^' {
                if index > 0 {
                    counter[index - 1] += current_count;
                }
                if index < row.len() - 1 {
                    counter[index + 1] += current_count;
                }
                counter[index] = 0;
            }
        }
    }
    counter.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn verify_part_1() {
        let result = part_1_redux(SAMPLE);
        assert_eq!(21, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(40, result);
    }
}
