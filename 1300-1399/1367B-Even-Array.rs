use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for i in 0..t {
        let mut n_input = String::new();
        io::stdin().read_line(&mut n_input).expect("Failed to read line");
        let n: usize = n_input.trim().parse().expect("Invalid n");

        let mut arr_input = String::new();
        io::stdin().read_line(&mut arr_input).expect("Failed to read line");

        let arr: Vec<i32> = arr_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        let mut misplaced_even = 0;
        let mut misplaced_odd = 0;

        for i in 0..n {
            let i_parity = i % 2;
            let val_parity = (arr[i] % 2).abs() as usize;

            if i_parity != val_parity {
                if i_parity == 0 {
                    misplaced_even += 1;
                } else {
                    misplaced_odd += 1;
                }
            }
        }

        if misplaced_even != misplaced_odd {
            println!("-1");
        } else {
            println!("{}", misplaced_even);
        }
    }
}
