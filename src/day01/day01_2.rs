use aoc::read_lines;

fn main() {
    let mut increased = 0;
    let mut last = u32::MAX;
    let mut window = [u32::MAX, u32::MAX, u32::MAX];
    let mut window_ind = 0;

    read_lines("src/day01/input.in", |line| {
        let i = line.parse::<u32>().unwrap();
        window[window_ind] = i;
        window_ind = (window_ind + 1) % 3;

        let sum = window.iter().fold(0 as u32, |acc, x| acc.saturating_add(*x));
        if sum < u32::MAX {
            increased += (sum > last) as u32;
            last = sum;
        }
    }).unwrap();

    println!("{}", increased);
}
