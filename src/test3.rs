#[test]
fn test31() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
#[test]
fn test32() {
    // Оголошуємо змінну і робимо її змінною
    let mut x = 1;

    // Змінюємо значення змінної
    x += 2;

    // Перевіряємо результат
    assert_eq!(x, 3);

    // Виводимо повідомлення про успіх
    println!("Success!");
}


