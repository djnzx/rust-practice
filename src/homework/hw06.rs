fn main() {
    let levels = 5;
    let max_width = 1 + 2 * (levels - 1);

    for level in 1..=levels {
        println!("{:^width$}", "*", width = max_width);

        for row in 0..level {
            let stars = "*".repeat(1 + 2 * row);
            println!("{:^width$}", stars, width = max_width);
        }
    }
}
