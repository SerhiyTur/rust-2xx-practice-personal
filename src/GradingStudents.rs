use std::io;

fn round_grade(grade: i32) -> i32 {
    if grade < 38 {
        grade
    } else {
        let next_multiple_of_5 = ((grade / 5) + 1) * 5;
        if next_multiple_of_5 - grade < 3 {
            next_multiple_of_5
        } else {
            grade
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut grades = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let grade: i32 = input.trim().parse().unwrap();
        grades.push(round_grade(grade));
    }

    for grade in grades {
        println!("{}", grade);
    }
}
