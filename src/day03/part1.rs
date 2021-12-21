use aoc::read_lines;

fn main() {
    // will store number of 1s in each position
    // index 0 -> LSB
    // index 31 -> MSB
    let mut num_bits = [0u32; 32];
    let mut num_num = 0u32;

    read_lines("src/day03/input.in", |line| {
        let mut num = u32::from_str_radix(line.as_str(), 2).unwrap();

        let mut i = 0;
        while num > 0 {
            // increment if bit is 1
            num_bits[i] += num & 1;

            // move next bit to LSB
            num >>= 1;

            i += 1;
        }

        num_num += 1;
    }).unwrap();

    // to check if number of set bits is more than half
    num_num /= 2;

    let mut γ = 0;
    for i in (0..32).rev() {
        // make space in LSB for new bit
        γ <<= 1;
        // set LSB if number of set bits in this position is more than majority
        //   else leave it 0
        γ |= (num_bits[i] > num_num) as u32;
    }

    // set everything after MSB of γ to 1s
    // consider γ = 1000 0000 0000 0000 0000 0000 0000 0000
    let mut mask = γ;
    // γ becomes 1100 0000 0000 0000 0000 0000 0000 0000
    mask |= mask >> 1;
    // γ becomes 1111 0000 0000 0000 0000 0000 0000 0000
    mask |= mask >> 2;
    // γ becomes 1111 1111 0000 0000 0000 0000 0000 0000
    mask |= mask >> 4;
    // γ becomes 1111 1111 1111 1111 0000 0000 0000 0000
    mask |= mask >> 8;
    // γ becomes 1111 1111 1111 1111 1111 1111 1111 1111
    mask |= mask >> 16;

    // flip only those bits in γ which were set (everything after and including MSB)
    let ε = !γ & mask;

    println!("{}", γ * ε);
}
