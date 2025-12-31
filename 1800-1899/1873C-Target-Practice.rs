use std::io;
use std::cmp::min;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut point: u32 = 0;

        for i in 0..10 {
            let mut row = String::new();
            io::stdin().read_line(&mut row).expect("Failed to read line");
            let row = row.trim();

            let row_chars: Vec<char> = row.chars().collect();

            for j in 0..10 {
                if row_chars[j] == 'X' {
                    let distance = min(min(i, j), min(9 - i, 9 - j));
                    point += (distance + 1) as u32;
                }
            }
        }

        println!("{}", point);
    }
}
