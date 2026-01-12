use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut n_input = String::new();
        io::stdin()
            .read_line(&mut n_input)
            .expect("Failed to read line");
        let _n: usize = n_input.trim().parse().expect("Please enter a number");

        let mut arr_input = String::new();
        io::stdin()
            .read_line(&mut arr_input)
            .expect("Failed to read line");

        let arr: Vec<i32> = arr_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        let total: i32 = arr.iter().sum();
        let has_even = arr.iter().any(|&x| x % 2 == 0);
        let has_odd = arr.iter().any(|&x| x % 2 == 1);

        if total % 2 == 1 {
            println!("YES");
        } else if has_even && has_odd {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
