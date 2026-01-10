use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let nums: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let a = nums[0];
        let b = nums[1];
        let diff = b - a;

        if diff == 0 {
            println!("0");
        } else if diff > 0 {
            println!("{}", if diff % 2 == 1 { 1 } else { 2 });
        } else {
            let diff_abs = diff.abs();
            println!("{}", if diff_abs % 2 == 0 { 1 } else { 2 });
        }
    }
}
