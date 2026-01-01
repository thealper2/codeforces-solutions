use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut row = String::new();
        io::stdin().read_line(&mut row).expect("Failed to read line");
        let row = row.trim();

        let row_chars: Vec<char> = row.chars().collect();
        let mut a_count: i64 = 0;

        for i in 0..5 {
            if row_chars[i] == 'A' {
                a_count += 1;
            } else {
                a_count -= 1;
            }
        }

        if a_count > 0 {
            println!("A");
        } else {
            println!("B");
        }
    }
}
