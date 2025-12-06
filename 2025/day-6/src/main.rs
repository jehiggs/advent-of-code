use aoc_lib::runner;
use std::{char, error::Error};

const INPUT: &str = "./2025/day-6/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    let mut calculations: Vec<Vec<_>> = Vec::new();
    let mut total = 0;
    let mut current_index = 0;
    for token in Lexer::new(input) {
        match token {
            Token::Digit(number) => {
                if let Some(calc) = calculations.get_mut(current_index) {
                    calc.push(number);
                } else {
                    calculations.push(Vec::from([number]));
                }
                current_index += 1;
            }
            Token::NewLine => {
                current_index = 0;
            }
            Token::Add => {
                if let Some(calc) = calculations.get(current_index) {
                    total += calc.iter().sum::<usize>();
                }
                current_index += 1;
            }
            Token::Mul => {
                if let Some(calc) = calculations.get(current_index) {
                    total += calc.iter().product::<usize>();
                }
                current_index += 1;
            }
        }
    }
    total
}

fn part_2(input: &str) -> usize {
    let columns = columns(input);
    let tokens = Lexer::new(&columns);
    let mut total = 0;
    let mut current_calc = Vec::new();
    for token in tokens {
        match token {
            Token::Digit(number) => {
                current_calc.push(number);
            }
            Token::NewLine => {}
            Token::Add => {
                total += current_calc.iter().sum::<usize>();
                current_calc.clear();
            }
            Token::Mul => {
                total += current_calc.iter().product::<usize>();
                current_calc.clear();
            }
        }
    }
    total
}

enum Token {
    Digit(usize),
    Add,
    Mul,
    NewLine,
}

struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer { input }
    }

    fn consume_whitespace(&mut self) {
        let next = self
            .input
            .find(|c: char| !c.is_ascii_whitespace() || c == '\n');
        if let Some(index) = next {
            self.input = &self.input[index..];
        } else {
            self.input = &self.input[self.input.len()..self.input.len()];
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_whitespace();
        match self.input.chars().nth(0) {
            Some(num) if num.is_numeric() => {
                let end = self
                    .input
                    .find(|c: char| !c.is_numeric())
                    .unwrap_or(self.input.len());
                let output = self.input[0..end]
                    .parse::<usize>()
                    .expect("Should be an integer");
                self.input = &self.input[end..];
                Some(Token::Digit(output))
            }
            Some('\n') => {
                self.input = &self.input[1..];
                Some(Token::NewLine)
            }
            Some('*') => {
                self.input = &self.input[1..];
                Some(Token::Mul)
            }
            Some('+') => {
                self.input = &self.input[1..];
                Some(Token::Add)
            }
            Some(_) => {
                panic!("Unexpected token")
            }
            None => None,
        }
    }
}

fn columns(input: &str) -> String {
    let row_length = input
        .split_once('\n')
        .map(|(first, _)| first.len() + 1)
        .expect("Should be at least one line");
    let mut columns = String::new();
    for i in 0..row_length - 1 {
        for j in (i..input.len()).step_by(row_length) {
            columns.push(input.chars().nth(j).expect("Should be a character"));
        }
        columns.push('\n');
    }
    columns.split('\n').rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(4_277_556, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(3_263_827, result);
    }
}
