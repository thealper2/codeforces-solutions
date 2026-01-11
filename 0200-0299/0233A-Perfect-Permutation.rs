use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    if n % 2 == 1 {
        println!("-1");
        return;
    }

    let mut result: Vec<usize> = (1..=n).collect();

    for i in (0..n).step_by(2) {
        result.swap(i, i + 1);
    }

    let output: Vec<String> = result.iter().map(|x| x.to_string()).collect();
    println!("{}", output.join(" "));
}
