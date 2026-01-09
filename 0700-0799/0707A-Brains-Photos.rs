use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let parts: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let n = parts[0];
    let _m = parts[1];

    for _ in 0..n {
        let mut row_input = String::new();
        io::stdin()
            .read_line(&mut row_input)
            .expect("Failed to read line");

        for pixel in row_input.trim().split_whitespace() {
            if pixel == "C" || pixel == "M" || pixel == "Y" {
                println!("#Color");
                return;
            }
        }
    }

    println!("#Black&White");
}
