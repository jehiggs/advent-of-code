use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

// Input: lines of digits like 531254136523
// Task: find the highest 2 digit number that can be made by selecting two digits in order.
// e.g. for the given input, the highest 2 digit number is 65
// Output: Sum of the highest 2 digit numbers that can be made in each row of the input.
fn main() -> Result<(), Box<dyn Error>> {
    let part_1_start = Instant::now();
    let reader = BufReader::new(File::open("./2025/day-3/input.txt")?);
    let input = reader.lines().map_while(Result::ok);
    let part_1 = part_1(input);
    let part_1_end = Instant::now();
    println!("Part 1: Sum is {part_1}");
    println!(
        "Part 1 took {}s.",
        (part_1_end - part_1_start).as_secs_f64()
    );

    let part_2_start = Instant::now();
    let reader = BufReader::new(File::open("./2025/day-3/input.txt")?);
    let input = reader.lines().map_while(Result::ok);
    let part_2 = part_2(input);
    let part_2_end = Instant::now();
    println!("Part 2: Sum is {part_2}");
    println!(
        "Part 2 took {}s.",
        (part_2_end - part_2_start).as_secs_f64()
    );
    Ok(())
}

fn part_1<T: AsRef<str>>(input: impl Iterator<Item = T>) -> usize {
    solve(input, 2)
}

fn part_2<T: AsRef<str>>(input: impl Iterator<Item = T>) -> usize {
    solve(input, 12)
}

fn solve<T: AsRef<str>>(input: impl Iterator<Item = T>, length: usize) -> usize {
    input.fold(0, |acc, item| {
        let mut result = 0;
        for i in 0..item.as_ref().len() - length + 1 {
            let value = &item.as_ref()[i..i + length]
                .parse::<usize>()
                .expect("Should be a number");
            for j in (1..length + 1).rev() {
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

    #[test]
    fn verify_part_1() {
        let sample = "987654321111111
        811111111111119
        234234234234278
        818181911112111";
        let result = part_1(sample.split('\n').map(|item| item.trim_start()));
        assert_eq!(357, result);
    }

    #[test]
    fn verify_part_2() {
        let sample = "987654321111111
        811111111111119
        234234234234278
        818181911112111";
        let result = part_2(sample.split('\n').map(|item| item.trim_start()));
        assert_eq!(3_121_910_778_619, result);
    }
}
