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

#[test]
fn test33() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // Щоб уникнути помилки, просто виведемо тільки x
    println!("The value of x is {}", x);
}
#[test]
fn test34() {
    let x = "hello"; // Оголошуємо змінну x безпосередньо у функції main
    println!("{}, world", x); // Використовуємо змінну x
}

fn define_x() {
    let _x = "hello"; // Оголошуємо змінну _x, але не використовуємо її
}






