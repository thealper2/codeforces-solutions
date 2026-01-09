use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        let s = s.trim();

        if s.len() >= 2 {
            let p = format!("{}i", &s[..s.len() - 2]);
            println!("{}", p);
        } else {
            println!("i");
        }
    }
}
