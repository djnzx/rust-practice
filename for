fn main() {
    const SIZE: usize = 5; // Розмір ромба

    // Верхня частина ромба
    for i in 0..SIZE {
        let spaces = " ".repeat(SIZE - i - 1);
        let stars = "*".repeat(2 * i + 1);
        print!("{}{}", spaces, stars);
        println!();
    }

    // Нижня частина ромба
    for i in (0..SIZE - 1).rev() {
        let spaces = " ".repeat(SIZE - i - 1);
        let stars = "*".repeat(2 * i + 1);
        print!("{}{}", spaces, stars);
        println!();
    }
}
