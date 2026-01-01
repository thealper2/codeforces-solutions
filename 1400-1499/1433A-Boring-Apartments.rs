use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..n {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read line");
        let num = num.trim();

        let l = num.len();

        let d: u32 = num.chars().next().unwrap().to_digit(10).unwrap();

        let mut total = 0;

        for i in 1..d {
            total += 10;
        }

        for i in 1..=l {
            total += i;
        }

        println!("{}", total);
    }
}
