use aoc::read_lines;

fn main() {
    let mut increased = 0;
    let mut last = u32::MAX;

    read_lines("src/day01/input.in", |line| {
        let i = line.parse::<u32>().unwrap();
        increased += (i > last) as u32;
        last = i;
    }).unwrap();

    println!("{}", increased);
}
