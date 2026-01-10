use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let parts: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        let min_ = parts.iter().min().unwrap();
        let max_ = parts.iter().max().unwrap();
        let result = (max_ - min_).abs();
        println!("{}", result);
    }
}
