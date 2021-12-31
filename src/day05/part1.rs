use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use aoc::read_string;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(",") {
            Ok(Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
        } else {
            Err("Could not find two parts for point".to_string())
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once(" -> ") {
            Ok(Line {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            })
        } else {
            Err("Could not find two parts for line".to_string())
        }
    }
}

fn get_range(start: usize, end: usize) -> RangeInclusive<usize> {
    if end > start {
        start..=end
    } else {
        end..=start
    }
}

impl Line {
    pub fn get_points(&self) -> Option<Vec<Point>> {
        if !self.is_straight() {
            return None;
        }

        if self.start.x == self.end.x {
            return Some(
                get_range(self.start.y, self.end.y)
                    .map(|y| Point { x: self.start.x, y })
                    .collect(),
            );
        } else {
            return Some(
                get_range(self.start.x, self.end.x)
                    .map(|x| Point { x, y: self.start.y })
                    .collect(),
            );
        }
    }

    pub fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

fn main() {
    let input = read_string("src/day05/input.in");

    let lines: Vec<Line> = input
        .lines()
        .filter(|s| s.trim() != "")
        .map(|x| x.parse().unwrap())
        .collect();

    // println!("{:#?}", lines);

    let mut map = HashMap::<Point, usize>::new();

    lines
        .into_iter()
        .filter(Line::is_straight)
        .for_each(|line| {
            line.get_points().unwrap().into_iter().for_each(|point| {
                // println!("x: {}, y: {}, collisions: {}", point.x, point.y, collisions);
                *map.entry(point).or_insert(0) += 1;
            })
        });

    let collisions = map.iter().filter(|(_, &count)| count > 1).count();

    println!("{}", collisions);
}
