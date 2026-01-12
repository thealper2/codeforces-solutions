use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    let n: usize = parts[0].parse().expect("Invalid n");
    let x: i64 = parts[1].parse().expect("Invalid x");

    let mut distressed = 0;
    let mut current = x;

    for _ in 0..n {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line = line.trim();

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            continue;
        }

        let sign = parts[0];
        let d: i64 = parts[1].parse().expect("Invalid amount");

        match sign {
            "+" => {
                current += d;
            }
            "-" => {
                if current >= d {
                    current -= d;
                } else {
                    distressed += 1;
                }
            }
            _ => {}
        }
    }

    println!("{} {}", current, distressed);
}
