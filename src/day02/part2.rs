use aoc::read_lines;

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    read_lines("src/day02/input.in", |line| {
        let splits: Vec<&str> = line.splitn(2, " ").collect();

        let direction = splits[0];
        let mag = splits[1];

        let mag: i32 = mag.parse().unwrap();

        match direction {
            "forward" => { x += mag; y += aim * mag; },
            "up" => { aim -= mag },
            "down" => { aim += mag },
            _ => {}
        }
    }).unwrap();

    println!("{}", x * y);
}
