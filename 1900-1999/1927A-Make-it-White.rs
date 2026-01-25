use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut n_input = String::new();
        io::stdin().read_line(&mut n_input).unwrap();
        let _n: usize = n_input.trim().parse().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();

        let first = s.find('B');
        let last = s.rfind('B');

        let result = match (first, last) {
            (Some(f), Some(l)) => l - f + 1,
            _ => 0,
        };

        println!("{}", result);
    }
}
