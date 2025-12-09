use aoc_lib::runner;
use std::error::Error;

const INPUT: &str = "./2025/day-9/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    runner::run("Part 1", INPUT, part_1)?;
    runner::run("Part 2", INPUT, part_2)?;
    Ok(())
}

fn part_1(input: &str) -> usize {
    parse(input)
        .map(|point| {
            parse(input)
                .map(|other| {
                    Rect::new([
                        point,
                        Point::new(point.x, other.y),
                        other,
                        Point::new(other.x, point.y),
                    ])
                    .area()
                })
                .max()
                .expect("Should be an element")
        })
        .max()
        .expect("Should be an element")
}

fn part_2(input: &str) -> usize {
    let perimeter: Vec<_> = parse(input)
        .loop_pairs()
        .map(|(a, b)| Line::new(a, b))
        .collect();
    parse(input)
        .map(|point| {
            parse(input)
                .map(move |other| {
                    Rect::new([
                        point,
                        Point::new(point.x, other.y),
                        other,
                        Point::new(other.x, point.y),
                    ])
                })
                .filter(|rect| rect.in_perimeter(&perimeter))
                .map(|rect| rect.area())
                .max()
                .unwrap_or(0)
        })
        .max()
        .expect("Should be an element")
}

fn parse(input: &str) -> impl Iterator<Item = Point> {
    input.split('\n').map(|line| {
        let elems = line.split_once(',').expect("Should be comma separated");
        Point::new(
            elems.0.parse().expect("Number"),
            elems.1.parse().expect("Number"),
        )
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Line {
    start: Point,
    end: Point,
    x_aligned: bool,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        let x_aligned = start.y == end.y;
        Line {
            start,
            end,
            x_aligned,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        if self.start == *point || self.end == *point {
            return true;
        }
        if self.x_aligned {
            point.y == self.start.y && ((self.start.x <= point.x) ^ (self.end.x <= point.x))
        } else {
            point.x == self.start.x && ((self.start.y <= point.y) ^ (self.end.y <= point.y))
        }
    }

    fn intersects(&self, other: &Line) -> bool {
        if self.x_aligned == other.x_aligned {
            return false;
        }

        if self.contains(&other.start)
            || self.contains(&other.end)
            || other.contains(&self.start)
            || other.contains(&self.end)
        {
            return false;
        }

        if self.x_aligned {
            let other_x = other.start.x;
            let self_y = self.start.y;
            let i_x = (self.start.x <= other_x) ^ (self.end.x <= other_x);
            let i_y = (other.start.y <= self_y) ^ (other.end.y <= self_y);
            i_x && i_y
        } else {
            let self_x = self.start.x;
            let other_y = other.start.y;
            let i_x = (other.start.x <= self_x) ^ (other.end.x <= self_x);
            let i_y = (self.start.y <= other_y) ^ (self.end.y <= other_y);
            i_x && i_y
        }
    }
}

#[derive(Debug)]
struct Rect {
    points: [Point; 4],
    lines: [Line; 4],
}

impl Rect {
    fn new(points: [Point; 4]) -> Self {
        let lines = [
            Line::new(points[0], points[1]),
            Line::new(points[1], points[2]),
            Line::new(points[2], points[3]),
            Line::new(points[3], points[0]),
        ];
        Rect { points, lines }
    }

    fn in_perimeter(&self, perimeter: &[Line]) -> bool {
        if !self.points.iter().all(|point| {
            perimeter.iter().any(|l| l.contains(point))
                || [
                    Point::new(point.x - 0.5, point.y),
                    Point::new(point.x + 0.5, point.y),
                ]
                .iter()
                .map(|p| Line::new(Point::new(p.x, 0.), *p))
                .any(|ray| perimeter.iter().filter(|l| l.intersects(&ray)).count() % 2 == 1)
        }) {
            return false;
        }

        !self
            .lines
            .iter()
            .any(|line| perimeter.iter().any(|l| l.intersects(line)))
    }

    fn area(&self) -> usize {
        let max_x = self
            .points
            .iter()
            .map(|p| p.x as usize)
            .max_by_key(|p| *p)
            .expect("");
        let min_x = self
            .points
            .iter()
            .map(|p| p.x as usize)
            .min_by_key(|p| *p)
            .expect("");
        let max_y = self
            .points
            .iter()
            .map(|p| p.y as usize)
            .max_by_key(|p| *p)
            .expect("");
        let min_y = self
            .points
            .iter()
            .map(|p| p.y as usize)
            .min_by_key(|p| *p)
            .expect("");
        (max_x - min_x + 1) * (max_y - min_y + 1)
    }
}

#[derive(Debug)]
struct LoopPairs<I: Iterator<Item = Point>> {
    iterator: I,
    first: Point,
    previous: Point,
}

impl<I: Iterator<Item = Point>> Iterator for LoopPairs<I> {
    type Item = (Point, Point);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(point) => {
                let previous = self.previous;
                self.previous = point;
                Some((previous, point))
            }
            None => (self.first != self.previous).then(|| {
                let previous = self.previous;
                self.previous = self.first;
                (previous, self.first)
            }),
        }
    }
}

trait PairLoop {
    fn loop_pairs(self) -> LoopPairs<Self>
    where
        Self: Iterator<Item = Point>,
        Self: Sized;
}

impl<I: Iterator<Item = Point>> PairLoop for I {
    fn loop_pairs(mut self) -> LoopPairs<Self>
    where
        Self: Iterator<Item = Point>,
        Self: Sized,
    {
        let first = self.next().expect("Should be one element");
        LoopPairs {
            iterator: self,
            first,
            previous: first,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn verify_part_1() {
        let result = part_1(SAMPLE);
        assert_eq!(50, result);
    }

    #[test]
    fn verify_part_2() {
        let result = part_2(SAMPLE);
        assert_eq!(24, result);
    }

    #[test]
    fn intersects() {
        let l1 = Line::new(Point::new(2., 5.), Point::new(2., 3.));
        let l2 = Line::new(Point::new(9., 3.), Point::new(2., 3.));
        assert!(!l2.intersects(&l1));
    }

    #[test]
    fn contains() {
        let l = Line::new(Point::new(1., 5.), Point::new(1., 9.));
        let p = Point::new(1., 9.);
        assert!(l.contains(&p));
    }
}
