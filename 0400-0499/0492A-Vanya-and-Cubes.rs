use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n = input.trim().parse().expect("Please enter a number");
    let mut level = 1;
    let mut total_used = 0;

    loop {
        let needed = level * (level + 1) / 2;
        if total_used + needed > n {
            break;
        }

        total_used += needed;
        level += 1;
    }

    println!("{}", level - 1);
}
