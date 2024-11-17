use std::io::{self, Write};

fn main() {
    let mut n_input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n_input).unwrap();
    let n: usize = n_input.trim().parse().unwrap();

    let mut matrix = Vec::new();
    for _ in 0..n {
        let mut row_input = String::new();
        io::stdin().read_line(&mut row_input).unwrap();
        let row: Vec<i32> = row_input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix.push(row);
    }

    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += matrix[i][i];
        secondary_diagonal_sum += matrix[i][n - 1 - i];
    }

    let difference = (primary_diagonal_sum - secondary_diagonal_sum).abs();
    println!("{}", difference);
}
