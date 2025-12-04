use aoc_lib::runner;
use std::error::Error;

const INPUT: &str = "./2025/day-3/input.txt";

// Input: lines of digits like 531254136523
// Task: find the highest 2 digit number that can be made by selecting two digits in order.
// e.g. for the given input, the highest 2 digit number is 65
// Output: Sum of the highest 2 digit numbers that can be made in each row of the input.
fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    solve(input.split('\n'), 2)
}

fn part_2(input: &str) -> usize {
    solve(input.split('\n'), 12)
}

#[allow(clippy::cast_possible_truncation)]
fn solve<T: AsRef<str>>(input: impl Iterator<Item = T>, length: usize) -> usize {
    input.fold(0, |acc, item| {
        let mut result = 0;
        for i in 0..=item.as_ref().len() - length {
            let value = &item.as_ref()[i..i + length]
                .parse::<usize>()
                .expect("Should be a number");
            for j in (1..=length).rev() {
                let current = result % 10usize.pow(j as u32);
                let cur_value = value % 10usize.pow(j as u32);
                if cur_value > current {
                    result += cur_value - current;
                    break;
                }
            }
        }
        acc + result
    })
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    use crate::part_2;

    const SAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(357, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(3_121_910_778_619, result);
    }
}
