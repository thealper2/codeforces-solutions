use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut arr_input = String::new();
    io::stdin().read_line(&mut arr_input).unwrap();

    let arr: Vec<i32> = arr_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut max_len = 1;
    let mut curr_len = 1;

    for i in 1..n {
        if arr[i] > arr[i - 1] {
            curr_len += 1;
            if curr_len > max_len {
                max_len = curr_len;
            }
        } else {
            curr_len = 1;
        }
    }

    println!("{}", max_len);
}
