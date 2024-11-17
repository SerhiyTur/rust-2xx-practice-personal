use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let total_sum: u64 = numbers.iter().sum();
    let min_sum = total_sum - numbers.iter().max().unwrap();
    let max_sum = total_sum - numbers.iter().min().unwrap();

    println!("{} {}", min_sum, max_sum);
}
