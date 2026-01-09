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
        let mut n: i32 = n_input.trim().parse().expect("Please enter a number");

        let mut total = n / 4;
        n = n % 4;

        if n != 0 {
            total += n / 2;
        }

        println!("{}", total);
    }
}
