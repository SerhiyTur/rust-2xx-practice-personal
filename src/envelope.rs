const WIDTH: usize = 6;
const HEIGHT: usize = 3;
#[test]
fn main() {
    let mut envelope = String::new();

    for i in 0..HEIGHT {
        let outer_spaces = (WIDTH - 2 - i * 2) / 2; // Пробіли зліва та справа
        let spaces_inside = i * 2; // Пробіли всередині

        if i < HEIGHT / 2 {
            envelope.push_str(&format!(
                "{}*{}*{}",
                " ".repeat(outer_spaces),
                " ".repeat(spaces_inside),
                " ".repeat(outer_spaces)
            ));
        } else {
            envelope.push_str(&format!(
                "{}*{}*{}",
                " ".repeat(outer_spaces),
                " ".repeat(WIDTH - 2 - 2 * outer_spaces),
                " ".repeat(outer_spaces)
            ));
        }

        envelope.push('\n');
    }

    print!("{}", envelope);
    println!();  // Додамо ще одне перенесення рядка
}


