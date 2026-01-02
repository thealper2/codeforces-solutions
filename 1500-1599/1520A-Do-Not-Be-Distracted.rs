use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut n_input = String::new();
        io::stdin().read_line(&mut n_input).unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();

        let mut seen = HashSet::new();
        let mut prev = '\0';
        let mut ok = true;

        for c in s.chars() {
            if c != prev {
                if seen.contains(&c) {
                    ok = false;
                    break;
                }
                seen.insert(c);
                prev = c;
            }
        }

        println!("{}", if ok { "YES" } else { "NO" });
    }
}
