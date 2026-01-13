use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut keyboard = String::new();
        io::stdin().read_line(&mut keyboard).unwrap();
        let keyboard = keyboard.trim();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();

        let mut pos = HashMap::new();
        for (i, ch) in keyboard.chars().enumerate() {
            pos.insert(ch, i as i32);
        }

        let mut total_time = 0;
        let chars: Vec<char> = s.chars().collect();

        for i in 1..chars.len() {
            let prev = pos[&chars[i - 1]];
            let curr = pos[&chars[i]];
            total_time += (curr - prev).abs();
        }

        println!("{}", total_time);
    }
}
