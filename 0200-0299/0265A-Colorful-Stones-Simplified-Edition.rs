use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t = t.trim();

    let mut pos = 0;

    for instruction in t.chars() {
        if pos < s.len() {
            let s_char = s.chars().nth(pos).unwrap();
            if s_char == instruction {
                pos += 1;
            }
        }
    }

    println!("{}", pos + 1);
}
