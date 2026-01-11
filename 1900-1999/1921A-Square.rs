use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut points = Vec::new();

        for _ in 0..4 {
            let mut point_input = String::new();
            io::stdin()
                .read_line(&mut point_input)
                .expect("Failed to read line");

            let coords: Vec<i32> = point_input
                .trim()
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid coordinate"))
                .collect();

            points.push((coords[0], coords[1]));
        }

        let xs: Vec<i32> = points.iter().map(|&(x, _)| x).collect();

        let unique_xs: HashSet<i32> = xs.iter().cloned().collect();

        let mut unique_xs_vec: Vec<i32> = unique_xs.into_iter().collect();
        unique_xs_vec.sort();

        let side = (unique_xs_vec[1] - unique_xs_vec[0]).abs();
        let area = side * side;

        println!("{}", area);
    }
}
