use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut l_input = String::new();

        io::stdin()
            .read_line(&mut l_input)
            .expect("Failed to read line");
        let _l: usize = l_input.trim().parse().expect("Please enter a number");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line = line.trim();

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            continue;
        }

        let s = parts[0];
        let t = parts[1];

        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();

        s_chars.sort();
        t_chars.sort();

        if s_chars == t_chars {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
