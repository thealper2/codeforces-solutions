use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Please enter a number");

        let result = number / 2;
        println!("{}", result);
    }
}
