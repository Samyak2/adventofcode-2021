use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    let mut num_bits = [0u32; 5];
    let mut num_num = 0u32;

    loop {
        stdin.read_line(&mut buf).unwrap();

        let input = buf.trim();

        if input == "" {
            break;
        }

        let num = u32::from_str_radix(input, 2).unwrap();

        num_bits[0] += (num & 0b10000) >> 4;
        num_bits[1] += (num & 0b01000) >> 3;
        num_bits[2] += (num & 0b00100) >> 2;
        num_bits[3] += (num & 0b00010) >> 1;
        num_bits[4] += num & 0b00001;

        num_num += 1;

        buf.clear();
    }

    num_num /= 2;

    let γ = (((num_bits[0] > num_num) as u32) << 4)
        | (((num_bits[1] > num_num) as u32) << 3)
        | (((num_bits[2] > num_num) as u32) << 2)
        | (((num_bits[3] > num_num) as u32) << 1)
        | ((num_bits[4] > num_num) as u32);
    // let ε = (((num_bits[0] < num_num) as u32) << 4)
    //     | (((num_bits[1] < num_num) as u32) << 3)
    //     | (((num_bits[2] < num_num) as u32) << 2)
    //     | (((num_bits[3] < num_num) as u32) << 1)
    //     | ((num_bits[4] < num_num) as u32);
    let ε = !γ & 0b11111;

    // println!("{}, {}, {:?}, {}", γ, ε, num_bits, num_num);

    println!("{}", γ * ε);
}
