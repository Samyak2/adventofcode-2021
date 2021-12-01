use std::io;

fn part1() {
    let mut last = u32::MAX;
    let mut increased = 0;
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        stdin.read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if trimmed == "" {
            break;
        }

        let i = trimmed.parse::<u32>().unwrap();
        increased += (i > last) as u32;
        last = i;

        input.clear();
    }

    println!("{}", increased);
}

fn part2() {
    let mut increased = 0;
    let stdin = io::stdin();
    let mut input = String::new();
    let mut window = [u32::MAX, u32::MAX, u32::MAX];
    let mut window_ind = 0;
    let mut last = u32::MAX;

    loop {
        stdin.read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if trimmed == "" {
            break;
        }

        let i = trimmed.parse::<u32>().unwrap();

        window[window_ind] = i;
        window_ind = (window_ind + 1) % 3;

        // let sum = window[0].saturating_add(window[1]).saturating_add(window[2]);
        let sum = window.iter().fold(0 as u32, |acc, x| acc.saturating_add(*x));
        if sum < u32::MAX {
            increased += (sum > last) as u32;
            last = sum;
        }

        input.clear();
    }

    println!("{}", increased);
}

fn main() {
    part2();
}
