use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let t: usize = input.trim().parse().expect("Please enter a number");
    const TOTAL_MINUTES: i64 = 24 * 60;

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line.");
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let hours: i64 = parts[0].parse().expect("Invalid hours");
        let minutes: i64 = parts[1].parse().expect("Invalid minutes");
        let current_minutes = hours * 60 + minutes;
        println!("{}", TOTAL_MINUTES - current_minutes);

    }

}
