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
        let n: usize = n_input.trim().parse().expect("Please enter a number");

        let mut arr_input = String::new();
        io::stdin()
            .read_line(&mut arr_input)
            .expect("Failed to read line");

        let mut arr: Vec<i64> = arr_input
            .trim()
            .split_whitespace()
            .take(n)
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        let mut min_digit = 10;
        let mut min_index = 0;

        for (i, &digit) in arr.iter().enumerate() {
            if digit < min_digit {
                min_digit = digit;
                min_index = i;
            }
        }

        arr[min_index] += 1;

        let mut product: i64 = 1;

        for &digit in &arr {
            product *= digit;
        }

        println!("{}", product);
    }
}
