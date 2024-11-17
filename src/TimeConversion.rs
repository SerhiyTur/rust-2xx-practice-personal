use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();
    let period = &input[input.len() - 2..];
    let time = &input[..input.len() - 2];

    let mut parts = time.split(':');
    let hour: u32 = parts.next().unwrap().parse().unwrap();
    let minute = parts.next().unwrap();
    let second = parts.next().unwrap();

    let hour_24 = match period {
        "AM" if hour == 12 => 0,
        "PM" if hour != 12 => hour + 12,
        _ => hour,
    };

    println!("{:02}:{:}:{:}", hour_24, minute, second);
}
