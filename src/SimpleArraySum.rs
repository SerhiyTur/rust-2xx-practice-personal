use std::io;

fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = simple_array_sum(&numbers);
    println!("{}", result);
}

