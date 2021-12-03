use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    let mut x = 0;
    let mut y = 0;

    loop {
        stdin.read_line(&mut buf).unwrap();

        let input = buf.trim();

        if input == "" {
            break;
        }

        let splits: Vec<&str> = input.splitn(2, " ").collect();

        let direction = splits[0];
        let mag = splits[1];

        let mag: u32 = mag.parse().unwrap();

        match direction {
            "forward" => { x += mag },
            "up" => { y -= mag },
            "down" => { y += mag },
            _ => {}
        }

        buf.clear();
    }

    println!("{}", x * y);
}
