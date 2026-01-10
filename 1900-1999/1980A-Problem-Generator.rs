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

        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let _n: usize = parts[0].parse().expect("Invalid n");
        let m: usize = parts[1].parse().expect("Invalid m");

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        let s = s.trim();

        let mut counts = [0; 7];

        for ch in s.chars() {
            if ch >= 'A' && ch <= 'G' {
                let index = (ch as usize) - ('A' as usize);
                counts[index] += 1;
            }
        }

        let mut needed = 0;
        for i in 0..7 {
            if counts[i] < m {
                needed += m - counts[i];
            }
        }

        println!("{}", needed);
    }
}
