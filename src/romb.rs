const SIZE: usize = 5;
#[test]
fn test() {
    for i in 0..SIZE * 2 + 1 {
        let num_stars = if i < SIZE {
            2 * i + 1
        } else {
            2 * (SIZE * 2 - i) + 1
        };
        let spaces = SIZE * 2 - num_stars / 2;

        print!("{}{}",
               " ".repeat(spaces),
               "*".repeat(num_stars)
        );

        println!();
    }
}
