use std::io;

// note: there's probably a simpler solution with lesser loops
//     this is all I could come up with now

fn count_bits(nums: &Vec<u32>) -> [u32; 32] {
    // will store number of 1s in each position
    // index 0 -> LSB
    // index 31 -> MSB
    let mut num_bits = [0u32; 32];

    for n in nums {
        let mut num = *n;
        let mut i = 0;
        while num > 0 {
            // increment if bit is 1
            num_bits[i] += num & 1;

            // move next bit to LSB
            num >>= 1;

            i += 1;
        }
    }

    num_bits
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    let mut nums = Vec::<u32>::new();

    loop {
        stdin.read_line(&mut buf).unwrap();

        let input = buf.trim();

        if input == "" {
            break;
        }

        let num = u32::from_str_radix(input, 2).unwrap();
        nums.push(num);

        buf.clear();
    }

    let mut nums2 = nums.clone();
    let num_bits_orig = count_bits(&nums);

    for i in (0..32).rev().filter(|i| num_bits_orig[*i] > 0) {
        let num_bits = count_bits(&nums2);
        let mask = ((num_bits[i] >= (nums2.len() as u32 + 1) / 2) as u32) << i;

        nums2 = nums2.into_iter().filter(|x| ((x ^ mask) & (1 << i)) == 0).collect();

        if nums2.len() == 1 {
            break;
        }
    }

    for i in (0..32).rev().filter(|i| num_bits_orig[*i] > 0) {
        let num_bits = count_bits(&nums);
        let mask = ((num_bits[i] < (nums.len() as u32 + 1) / 2) as u32) << i;

        nums = nums.into_iter().filter(|x| ((x ^ mask) & (1 << i)) == 0).collect();

        if nums.len() == 1 {
            break;
        }
    }

    println!("{}", nums2[0] * nums[0]);
}
