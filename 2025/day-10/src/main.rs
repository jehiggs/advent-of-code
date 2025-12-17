use aoc_lib::runner;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::mem;

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
    buttons: Vec<u16>,
    joltages: Vec<u32>,
}

impl<T: AsRef<str>> From<T> for Machine {
    fn from(value: T) -> Self {
        let mut target_lights = 0;
        let mut buttons = Vec::new();
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
                    buttons.push(button);
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
        let mut cache = HashMap::new();
        self.minimum_joltage_buttons(&self.joltages, &mut cache)
    }

    fn minimum_joltage_buttons(
        &self,
        joltages: &[u32],
        map: &mut HashMap<Vec<u32>, usize>,
    ) -> usize {
        if map.contains_key(joltages) {
            map[joltages]
        } else if joltages.iter().all(|j| *j == 0) {
            0
        } else if joltages.iter().all(|j| j % 2 == 0) {
            let mut combinations = Vec::new();
            combinations.push(Vec::new());
            combinations.extend(self.buttons_for_pattern(0));
            let extra = self
                .buttons
                .iter()
                .map(|button| Vec::from([*button, *button]));
            combinations.extend(extra);
            let result = combinations
                .iter()
                .filter(|combination| {
                    !joltages.iter().enumerate().any(|(index, j)| {
                        combination
                            .iter()
                            .filter(|button| *button & (0b1 << index) == 0b1 << index)
                            .count()
                            > *j as usize
                    })
                })
                .map(|combination| {
                    let new_joltages: Vec<_> = joltages
                        .iter()
                        .enumerate()
                        .map(|(index, j)| {
                            let mut jolt = *j;
                            for button in combination {
                                if button & (0b1 << index) == 0b1 << index {
                                    jolt -= 1;
                                }
                            }
                            jolt / 2
                        })
                        .collect();
                    self.minimum_joltage_buttons(&new_joltages, map)
                        .saturating_mul(2)
                        .saturating_add(combination.len())
                })
                .min()
                .unwrap_or(usize::MAX);
            map.insert(joltages.to_vec(), result);
            result
        } else {
            let pattern = joltages
                .iter()
                .enumerate()
                .map(|(index, j)| if j % 2 == 1 { 0b1 << index } else { 0 })
                .sum();
            let combinations = self.buttons_for_pattern(pattern);
            let result = combinations
                .iter()
                .filter(|combination| {
                    !joltages.iter().enumerate().any(|(index, j)| {
                        combination
                            .iter()
                            .filter(|button| *button & (0b1 << index) == 0b1 << index)
                            .count()
                            > *j as usize
                    })
                })
                .map(|combination| {
                    let new_joltages: Vec<_> = joltages
                        .iter()
                        .enumerate()
                        .map(|(index, j)| {
                            let mut jolt = *j;
                            for button in combination {
                                if button & (0b1 << index) == 0b1 << index {
                                    jolt -= 1;
                                }
                            }
                            jolt
                        })
                        .collect();
                    self.minimum_joltage_buttons(&new_joltages, map)
                        .saturating_add(combination.len())
                })
                .min()
                .unwrap_or(usize::MAX);
            map.insert(joltages.to_vec(), result);
            result
        }
    }

    fn buttons_for_pattern(&self, pattern: u16) -> Vec<Vec<u16>> {
        let mut result = Vec::new();
        for k in 1..=self.buttons.len() {
            let mut indices: Vec<_> = (0..k).collect();
            let mut remaining_indices = true;
            while remaining_indices {
                let buttons: Vec<_> = indices.iter().map(|i| self.buttons[*i]).collect();
                if buttons
                    .iter()
                    .copied()
                    .reduce(|a, b| a ^ b)
                    .expect("Number")
                    == pattern
                {
                    result.push(buttons);
                }
                // Increment indices
                let mut i = k - 1;
                while indices[i] == self.buttons.len() + i - k {
                    if i > 0 {
                        i -= 1;
                    } else {
                        remaining_indices = false;
                        break;
                    }
                }

                if remaining_indices {
                    indices[i] += 1;
                    let mut j = i + 1;
                    while j < k {
                        indices[j] = indices[j - 1] + 1;
                        j += 1;
                    }
                }
            }
        }
        result
    }

    // This sadly does not work just yet.
    // I believe the Gauss-Jordan elimination code does work. However, the
    // test for valid solutions is not quite right - the free variable substitution is clearly incorrect,
    // and this leads to an overcount as it rejects otherwise valid solutions.
    #[allow(unused)]
    fn min_joltages_via_gauss_jordan(&self) -> usize {
        let entries: Vec<Vec<_>> = self
            .joltages
            .iter()
            .enumerate()
            .map(|(i, joltage)| {
                let mut row: Vec<f64> = self
                    .buttons
                    .iter()
                    .map(|button| {
                        if *button & (0b1 << i) == (0b1 << i) {
                            1.
                        } else {
                            0.
                        }
                    })
                    .collect();
                row.push(f64::from(*joltage));
                row
            })
            .collect();
        let matrix = Matrix::new(entries);
        let biggest_button = self
            .buttons
            .iter()
            .map(|b| {
                let mut start = *b;
                let mut count = 0;
                while start > 0 {
                    count += 1;
                    start = start & (start - 1);
                }
                count
            })
            .max()
            .expect("Should be an element");
        let count = self.joltages.iter().sum::<u32>();
        let min = count / biggest_button + 1;
        for guess in min..count {
            let mut extra_constraint: Vec<_> = (0..matrix.columns).map(|_| 1.).collect();
            extra_constraint.push(f64::from(guess));
            let mut matrix_to_test = matrix.clone();
            matrix_to_test.data.push(extra_constraint);
            matrix_to_test.rows += 1;
            matrix_to_test.reduce();
            if matrix_to_test.data.iter().all(|row| row_valid(row))
                && matrix_to_test.valid_solution(guess)
            {
                return guess as usize;
            }
        }
        count as usize
    }
}

fn row_valid(row: &[f64]) -> bool {
    if row.iter().all(|n| n.round() == 0.) {
        return true;
    }
    if row[0..row.len() - 1].iter().all(|n| n.round() == 0.) && row[row.len() - 1].round() != 0. {
        return false;
    }
    row.iter().all(|n| *n >= 0.)
        || row.iter().all(|n| *n <= 0.)
        || !(row[0..row.len() - 1].iter().all(|n| *n >= 0.)
            || row[0..row.len() - 1].iter().all(|n| *n <= 0.))
}

#[derive(Debug, Clone)]
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
                    if self.data[i][column].abs() > self.data[j][column].abs() {
                        i
                    } else {
                        j
                    }
                })
                .expect("Should be an element");
            if self.data[row_max][column] == 0. {
                column += 1;
            } else {
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
        }
        // Backward substitution
        row = self.rows - 1;
        let mut complete = false;
        while !complete {
            if let Some((c, _)) = (0..self.columns)
                .map(|c| self.data[row][c])
                .enumerate()
                .find(|(_, f)| *f != 0.)
            {
                let scale = 1. / self.data[row][c];
                self.data[row][c] = 1.;
                for j in c + 1..=self.columns {
                    self.data[row][j] *= scale;
                }
                for r in 0..row {
                    let row_scale = self.data[r][c] / self.data[row][c];
                    self.data[r][c] = 0.;
                    for other_column in c + 1..=self.columns {
                        self.data[r][other_column] -= self.data[row][other_column] * row_scale;
                    }
                }
                if row == 0 || column == 0 {
                    complete = true;
                } else {
                    row -= 1;
                    column -= 1;
                }
            } else {
                // All entries are 0.
                row -= 1;
            }
        }
    }

    fn valid_solution(&self, guess: u32) -> bool {
        let zero_rows = self
            .data
            .iter()
            .filter(|row| row.iter().all(|n| n.round() == 0.))
            .count();
        if self.rows - zero_rows >= self.columns {
            return true;
        }
        let free_vars = self.columns + zero_rows - self.rows;
        let mut current_vars: Vec<u32> = (0..free_vars).map(|_| 0).collect();
        while current_vars.iter().sum::<u32>() < guess {
            if self.data.iter().all(|row| {
                let mut total = row[row.len() - 1];
                for i in self.columns - free_vars..self.columns {
                    total -= row[i] * f64::from(current_vars[i + free_vars - self.columns]);
                }
                total >= 0. && total - total.fract() < 1e-10
            }) {
                return true;
            }
            // The variable substitution here is incorrect.
            for var in &mut current_vars {
                *var += 1;
            }
        }
        false
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
}
