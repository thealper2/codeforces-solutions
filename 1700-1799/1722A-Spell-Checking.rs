use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut length_input = String::new();
        io::stdin().read_line(&mut length_input).expect("Failed to read line");

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        let s = s.trim();

        let mut s_chars: Vec<char> = s.chars().collect();
        let mut timur_chars: Vec<char> = "Timur".chars().collect();

        s_chars.sort();
        timur_chars.sort();

        if s_chars == timur_chars {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
