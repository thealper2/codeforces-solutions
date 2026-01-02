use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut expression = String::new();
        io::stdin().read_line(&mut expression).expect("Failed to read line");
        let expression = expression.trim();

        let parts: Vec<&str> = expression.split('+').collect();

        let a: i32 = parts[0].parse().expect("Invalid number");
        let b: i32 = parts[1].parse().expect("Invalid number");

        let result = a + b;
        println!("{}", result);
    }
}
