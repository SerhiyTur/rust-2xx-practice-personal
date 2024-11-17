use std::io::{self, Write};

fn main() {
    let mut a_input = String::new();
    let mut b_input = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a_input).unwrap();
    io::stdin().read_line(&mut b_input).unwrap();

    let a: Vec<i32> = a_input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let b: Vec<i32> = b_input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut a_score = 0;
    let mut b_score = 0;
    for i in 0..3 {
        if a[i] > b[i] {
            a_score += 1;
        } else if a[i] < b[i] {
            b_score += 1;
        }
    }

    // Виведення результату
    println!("{} {}", a_score, b_score);
}
