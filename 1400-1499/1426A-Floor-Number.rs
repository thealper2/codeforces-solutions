use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut parts_input = String::new();
        io::stdin()
            .read_line(&mut parts_input)
            .expect("Failed to read line");

        let parts: Vec<i32> = parts_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        let mut n = parts[0];
        let x = parts[1];

        if n <= 2 {
            println!("1");
            continue;
        }

        n -= 2;
        let floor: i32 = (n + x - 1) / x + 1;
        println!("{}", floor);
    }
}
