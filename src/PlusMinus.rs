use std::io;

fn main() {
    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).unwrap();
    let _n: usize = n_input.trim().parse().unwrap();

    let mut arr_input = String::new();
    io::stdin().read_line(&mut arr_input).unwrap();


    let arr: Vec<i32> = arr_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;

    for &num in &arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    let total = arr.len() as f64;

    let positive_ratio = positive_count as f64 / total;
    let negative_ratio = negative_count as f64 / total;
    let zero_ratio = zero_count as f64 / total;


    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}
