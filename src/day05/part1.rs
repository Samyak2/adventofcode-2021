mod lib;

use std::collections::HashMap;

use aoc::read_string;
use lib::{Line, Point};

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
