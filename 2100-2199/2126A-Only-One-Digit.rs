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
        let n: i64 = n_input.trim().parse().expect("Please enter a number");

        let mut min_d = 9;

        let mut power = 1;

        while power <= n {
            let digit = (n / power) % 10;

            if digit < min_d {
                min_d = digit;
            }

            power *= 10;
        }

        println!("{}", min_d);
    }
}
