fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let count = |center: i32, deltas: &[i32]|
        deltas
            .iter()
            .map(|d| d + center)
            .filter(|&d| d >= s && d <= t)
            .count();

    let count_apples = count(a, apples);
    let count_oranges = count(b, oranges);

    println!("{count_apples}\n{count_oranges}");
}
