use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    loop {
        stdin.read_line(&mut buf).unwrap();

        let input = buf.trim();

        if input == "" {
            break;
        }

        let splits: Vec<&str> = input.splitn(2, " ").collect();

        let direction = splits[0];
        let mag = splits[1];

        let mag: i32 = mag.parse().unwrap();

        match direction {
            "forward" => { x += mag; y += aim * mag; },
            "up" => { aim -= mag },
            "down" => { aim += mag },
            _ => {}
        }

        buf.clear();
    }

    println!("{}", x * y);
}
