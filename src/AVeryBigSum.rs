use std::io::{self, Write};

fn main() {
    let mut n_input = String::new();
    let mut numbers_input = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n_input).unwrap();

    io::stdin().read_line(&mut numbers_input).unwrap();

    let numbers: Vec<i64> = numbers_input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let sum: i64 = numbers.iter().sum();

    println!("{}", sum);
}
