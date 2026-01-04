use std::cmp;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line = line.trim();

        let parts: Vec<&str> = line.split_whitespace().collect();

        let a: i64 = parts[0].parse().expect("Invalid number");
        let b: i64 = parts[1].parse().expect("Invalid number");

        let side1 = cmp::max(2 * cmp::min(a, b), cmp::max(a, b));
        let side2 = cmp::max(2 * a, 2 * b);
        let side3 = cmp::max(a + b, 2 * cmp::min(a, b));

        let result = cmp::min(side1 * side1, cmp::min(side2 * side2, side3 * side3));
        println!("{}", result);
    }
}
