use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::zip;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let part_1_start = Instant::now();
    let reader = BufReader::new(File::open("./2024/day-1/input.txt")?);
    let input = reader.lines().map_while(Result::ok);
    let part_1 = part_1(input);
    let part_1_end = Instant::now();
    println!("Part 1: Sum is {part_1}");
    println!(
        "Part 1 took {}s.",
        (part_1_end - part_1_start).as_secs_f64()
    );

    let part_2_start = Instant::now();
    let reader_2 = BufReader::new(File::open("./2024/day-1/input.txt")?);
    let input_2 = reader_2.lines().map_while(Result::ok);
    let part_2 = part_2(input_2);
    let part_2_end = Instant::now();
    println!("Part 2: Sum is {part_2}");
    println!(
        "Part 2 took {}s.",
        (part_2_end - part_2_start).as_secs_f64()
    );
    Ok(())
}

fn part_1<I: Iterator<Item = impl AsRef<str>>>(input: I) -> usize {
    let (mut first, mut second): (Vec<_>, Vec<_>) = input
        .map(|item| {
            item.as_ref()
                .split_once("   ")
                .map(|(a, b)| {
                    (
                        a.parse::<usize>().expect("Should be a number"),
                        b.parse::<usize>().expect("Should be a number"),
                    )
                })
                .expect("Should split at three spaces")
        })
        .unzip();
    first.sort_unstable();
    second.sort_unstable();
    zip(first, second).fold(0, |acc, (a, b)| acc + a.abs_diff(b))
}

fn part_2<I: Iterator<Item = impl AsRef<str>>>(input: I) -> usize {
    let (first, second): (Vec<_>, Vec<_>) = input
        .map(|item| {
            item.as_ref()
                .split_once("   ")
                .map(|(a, b)| {
                    (
                        a.parse::<usize>().expect("Should be a number"),
                        b.parse::<usize>().expect("Should be a number"),
                    )
                })
                .expect("Should split at three spaces")
        })
        .unzip();
    let occurrences =
        second
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<usize, usize>, number| {
                match acc.get_mut(number) {
                    Some(count) => *count += 1,
                    None => {
                        acc.insert(*number, 1);
                    }
                }
                acc
            });
    first.iter().fold(0, |acc, number| {
        acc + number * occurrences.get(number).unwrap_or(&0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn check_part_1() {
        let result = part_1(SAMPLE.split('\n'));
        assert_eq!(11, result);
    }

    #[test]
    fn check_part_2() {
        let result = part_2(SAMPLE.split('\n'));
        assert_eq!(31, result);
    }
}
