use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();

    if n % 2 == 1 {
        println!("Ehab");
    } else {
        println!("Mahmoud");
    }
}
