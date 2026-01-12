use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please enter a number");

    if n % 2 == 0 {
        let k = n / 2;
        println!("{}", k);

        let mut result = String::new();
        for _ in 0..(k - 1) {
            result.push_str("2 ");
        }

        result.push_str("2");

        println!("{}", result);
    } else {
        let k = (n - 3) / 2 + 1;
        println!("{}", k);

        let mut result = String::new();
        let count = (n - 3) / 2;
        for _ in 0..count {
            result.push_str("2 ");
        }

        result.push_str("3");

        println!("{}", result);
    }
}
