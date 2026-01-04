use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut arr_input = String::new();
        io::stdin().read_line(&mut arr_input).expect("Failed to read line");
        let arr_input = arr_input.trim();

        let arr: Vec<&str> = arr_input.split_whitespace().collect();

        let a: i64 = arr[0].parse().expect("Invalid number");
        let b: i64 = arr[1].parse().expect("Invalid number");

        if a > b {
            println!("{} {}", b, a);
        } else {
            println!("{} {}", a, b);
        }
    }
}
