use aoc_lib::runner;
use std::{collections::HashSet, error::Error, mem};

const INPUT: &str = "./2025/day-10/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    parse(input).map(|m| m.min_buttons()).sum()
}

fn part_2(input: &str) -> usize {
    parse(input).map(|m| m.min_joltages()).sum()
}

fn parse(input: &str) -> impl Iterator<Item = Machine> {
    input.split('\n').map(Machine::from)
}

#[derive(Debug)]
struct Machine {
    target_lights: u16,
    buttons: HashSet<u16>,
    joltages: Vec<u32>,
}

impl<T: AsRef<str>> From<T> for Machine {
    fn from(value: T) -> Self {
        let mut target_lights = 0;
        let mut buttons = HashSet::new();
        let mut joltages = Vec::new();
        for elem in value.as_ref().split(' ') {
            match elem.chars().nth(0) {
                Some('[') => {
                    target_lights = elem[1..elem.len() - 1]
                        .chars()
                        .enumerate()
                        .map(|(i, b)| if b == '#' { 0b1 << i } else { 0b0 })
                        .sum();
                }
                Some('(') => {
                    let button = elem[1..elem.len() - 1]
                        .split(',')
                        .map(|n| 0b1 << n.parse::<u32>().expect("Should be a number"))
                        .sum();
                    buttons.insert(button);
                }
                Some('{') => {
                    for num in elem[1..elem.len() - 1].split(',') {
                        joltages.push(num.parse().expect("Should be a number"));
                    }
                }
                _ => unreachable!("All elements should start with a bracket"),
            }
        }
        Machine {
            target_lights,
            buttons,
            joltages,
        }
    }
}

impl Machine {
    fn min_buttons(&self) -> usize {
        let mut current_count = 1;
        let mut values: HashSet<_> = self.buttons.iter().copied().collect();
        loop {
            if values.contains(&self.target_lights) {
                return current_count;
            }
            values = values
                .iter()
                .flat_map(|item| self.buttons.iter().map(|b| *item ^ b))
                .collect();
            current_count += 1;
        }
    }

    fn min_joltages(&self) -> usize {
        0
    }
}

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        // Augmented matrix, so ignore last element for columns during forward/backward substitution.
        let columns = data[0].len() - 1;
        Matrix {
            data,
            rows,
            columns,
        }
    }
    fn reduce(&mut self) {
        let mut row = 0;
        let mut column = 0;
        // Forward substitution
        while row < self.rows && column < self.columns {
            let row_max = (row..self.rows)
                .reduce(|i, j| {
                    if self.data[i][column] > self.data[j][column] {
                        i
                    } else {
                        j
                    }
                })
                .expect("Should be an element");
            if row_max > row {
                let (a, b) = self.data.split_at_mut(row_max);
                mem::swap(&mut a[row], &mut b[0]);
            }
            for r in row + 1..self.rows {
                let scale = self.data[r][column] / self.data[row][column];
                self.data[r][column] = 0.;
                for c in column + 1..=self.columns {
                    self.data[r][c] -= self.data[row][c] * scale;
                }
            }
            row += 1;
            column += 1;
        }
        // let position = min(row, column) - 1;
        // row = position;
        // column = position;
        // while row >= 0 && column >= 0 {

        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(7, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(33, result);
    }

    #[test]
    fn reduce() {
        let mut m = Matrix::new(vec![
            vec![0., 0., 0., 0., 1., 1., 3.],
            vec![0., 1., 0., 0., 0., 1., 5.],
            vec![0., 0., 1., 1., 1., 0., 4.],
            vec![1., 1., 0., 1., 0., 0., 7.],
            vec![1., 1., 1., 1., 1., 1., 10.],
        ]);
        m.reduce();
        println!("{m:?}");
    }
}
