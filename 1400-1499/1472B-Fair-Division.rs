use std::cmp::min;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut n_input = String::new();
        io::stdin()
            .read_line(&mut n_input)
            .expect("Failed to read line");
        let _n: usize = n_input.trim().parse().expect("Please enter a number");

        let mut candies_input = String::new();
        io::stdin()
            .read_line(&mut candies_input)
            .expect("Failed to read line");

        let candies: Vec<i32> = candies_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        let mut ones = 0;
        let mut twos = 0;

        for &candy in &candies {
            if candy == 1 {
                ones += 1;
            } else if candy == 2 {
                twos += 1;
            }
        }

        let total = ones + 2 * twos;

        if total % 2 != 0 {
            println!("NO");
            continue;
        }

        let half = total / 2;
        let twos_needed = min(twos, half / 2);
        let remaining = half - twos_needed * 2;

        if remaining <= ones {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
