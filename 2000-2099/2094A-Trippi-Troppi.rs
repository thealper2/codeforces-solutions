use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let mut word = String::from("");

        for i in 0..parts.len() {
            let c = parts[i].chars().nth(0).unwrap();
            word.push(c);
        }

        println!("{}", word);
    }
}
