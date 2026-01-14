use std::cmp::min;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let n = parts[0];
        let a = parts[1];
        let b = parts[2];

        let cost_individual = n * a;

        let pairs = n / 2;
        let remaining = n % 2;
        let cost_promotion = pairs * b + remaining * a;

        let result = min(cost_individual, cost_promotion);
        println!("{}", result);
    }
}
