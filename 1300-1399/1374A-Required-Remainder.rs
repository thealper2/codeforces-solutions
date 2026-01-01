use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");

        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() < 3 {
            continue;
        }

        let x: i64 = parts[0].parse().expect("Invalid x");
        let y: i64 = parts[1].parse().expect("Invalid y");
        let n: i64 = parts[2].parse().expect("Invalid n");

        if y >= x || y > n {
            println!("-1");
            continue;
        }

        let k = (n - y) / x;
        let result = k * x + y;
        println!("{}", result);
    }
}
