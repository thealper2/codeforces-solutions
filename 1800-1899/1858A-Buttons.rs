use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");

        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        let a: i64 = parts[0].parse().expect("Invalid a");
        let b: i64 = parts[1].parse().expect("Invalid b");
        let c: i64 = parts[2].parse().expect("Invalid c");

        if a > b {
            println!("First");
        } else if a < b {
            println!("Second");
        } else {
            if c % 2 == 0 {
                println!("Second");
            } else {
                println!("First");
            }
        }
    }
}
