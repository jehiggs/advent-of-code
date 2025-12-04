use aoc_lib::runner;
use std::error::Error;

const INPUT: &str = "./2025/day-4/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    let grid = parse(input.split('\n'));
    let row_len = grid.len();
    let column_len = grid[0].len();

    let mut count = 0;
    for i in 0..row_len {
        for j in 0..column_len {
            if grid[i][j] && count_adjacencies(i, j, row_len, column_len, &grid) < 4 {
                count += 1;
            }
        }
    }
    count
}

fn part_2(input: &str) -> usize {
    let mut grid = parse(input.split('\n'));
    let row_len = grid.len();
    let column_len = grid[0].len();

    let mut total_removed = 0;
    let mut removed_this_iter = true; // Start with removing items.
    while removed_this_iter {
        removed_this_iter = false;
        for i in 0..row_len {
            for j in 0..column_len {
                if grid[i][j] && count_adjacencies(i, j, row_len, column_len, &grid) < 4 {
                    total_removed += 1;
                    grid[i][j] = false;
                    removed_this_iter = true;
                }
            }
        }
    }
    total_removed
}

fn parse<I: Iterator<Item = impl AsRef<str>>>(input: I) -> Vec<Vec<bool>> {
    input
        .map(|line| line.as_ref().chars().map(|c| c == '@').collect())
        .collect()
}

#[allow(clippy::needless_range_loop)]
fn count_adjacencies(
    row: usize,
    column: usize,
    row_len: usize,
    column_len: usize,
    grid: &[Vec<bool>],
) -> usize {
    let row_min = if row == 0 { row } else { row - 1 };
    let row_max = if row == row_len - 1 { row } else { row + 1 };
    let column_min = if column == 0 { column } else { column - 1 };
    let column_max = if column == column_len - 1 {
        column
    } else {
        column + 1
    };

    let mut count = 0;
    for i in row_min..=row_max {
        for j in column_min..=column_max {
            if (i, j) != (row, column) && grid[i][j] {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(13, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(43, result);
    }
}
