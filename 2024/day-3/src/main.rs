use aoc_lib::runner;
use std::error::Error;

const INPUT: &str = "./2024/day-3/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    parse(input)
        .iter()
        .fold(0, |acc, pair| acc + (pair.0 * pair.1))
}

fn part_2(input: &str) -> usize {
    parse_2(input)
        .iter()
        .fold(0, |acc, pair| acc + (pair.0 * pair.1))
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut numbers = Vec::new();
    let mut data = input;
    while !data.is_empty() {
        data = consume_to_mul(data);
        if let Some(pair) = get_args(data) {
            numbers.push(pair);
        }
    }

    numbers
}

fn parse_2(input: &str) -> Vec<(usize, usize)> {
    let mut numbers = Vec::new();
    let mut data = input;
    let mut active = true;
    while !data.is_empty() {
        let (token, rest) = get_next_token(data);
        match token {
            "mul" => {
                if let Some(pair) = get_args(rest)
                    && active
                {
                    numbers.push(pair);
                }
            }
            "do()" => {
                active = true;
            }
            "don't()" => {
                active = false;
            }
            _ => {}
        }
        data = rest;
    }
    numbers
}

fn get_next_token(input: &str) -> (&str, &str) {
    let mul = input.find("mul(");
    let do_key = input.find("do()");
    let dont = input.find("don't()");
    let smallest = [mul, do_key, dont]
        .iter()
        .filter_map(|n| *n)
        .min()
        .unwrap_or(input.len());
    if let Some(idx) = mul
        && smallest == idx
    {
        (&input[smallest..smallest + 3], &input[smallest + 3..])
    } else if let Some(idx) = do_key
        && smallest == idx
    {
        (&input[smallest..smallest + 4], &input[smallest + 3..])
    } else if let Some(idx) = dont
        && smallest == idx
    {
        (&input[smallest..smallest + 7], &input[smallest + 7..])
    } else {
        (input, &input[smallest..])
    }
}

fn consume_to_mul(input: &str) -> &str {
    let idx = input.find("mul");
    let length = input.len();
    if let Some(i) = idx {
        &input[i + 3..length]
    } else {
        &input[length..length]
    }
}

fn get_args(input: &str) -> Option<(usize, usize)> {
    let mut string = input;
    if string.chars().next()? != '(' {
        return None;
    }
    string = &string[1..];
    let first_idx = get_number_index(string)?;
    let first = string[0..first_idx].parse::<usize>().ok()?;
    string = &string[first_idx..];
    if string.chars().next()? != ',' {
        return None;
    }
    string = &string[1..];
    let second_idx = get_number_index(string)?;
    let second = string[0..second_idx].parse::<usize>().ok()?;
    string = &string[second_idx..];
    if string.chars().next()? != ')' {
        return None;
    }
    Some((first, second))
}

fn get_number_index(input: &str) -> Option<usize> {
    input
        .chars()
        .enumerate()
        .find(|(_, c)| !c.is_numeric())
        .filter(|(idx, _)| *idx != 0)
        .map(|(idx, _)| idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const SAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(161, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE_2);
        assert_eq!(48, result);
    }
}
