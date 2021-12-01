use std::io;

fn main() {
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
