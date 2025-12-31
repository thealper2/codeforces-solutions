use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        let s = s.trim();

        let l = s.len();

        if l % 2 != 0 {
            println!("NO");
        } else {
            let half = l / 2;
            let first_half = &s[..half];
            let second_half = &s[half..];

            if first_half == second_half {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
