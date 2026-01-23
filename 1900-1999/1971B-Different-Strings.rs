use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        let s = s.trim();

        if s.len() == 1 {
            println!("NO");
            continue;
        }

        let first_char = s.chars().next().unwrap();
        let all_same = s.chars().all(|c| c == first_char);

        if all_same {
            println!("NO");
        } else {
            println!("YES");

            let mut chars: Vec<char> = s.chars().collect();

            for i in 1..chars.len() {
                if chars[i] != chars[0] {
                    chars.swap(0, i);
                    break;
                }
            }

            let result: String = chars.into_iter().collect();
            println!("{}", result);
        }
    }
}
