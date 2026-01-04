use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut n_input = String::new();
        io::stdin().read_line(&mut n_input).expect("Failed to read line");
        let n: usize = n_input.trim().parse().expect("Please enter a number");

        let mut arr_input = String::new();
        io::stdin().read_line(&mut arr_input).expect("Failed to read line");

        let mut arr: Vec<i32> = arr_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        arr.sort();
        let mut strictly_increasing = true;
        for i in 1..n {
            if arr[i] <= arr[i - 1] {
                strictly_increasing = false;
                break;
            }
        }

        println!("{}", if strictly_increasing { "YES" } else { "NO" });
    }
}
