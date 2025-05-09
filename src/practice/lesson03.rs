fn countApplesAndOranges(
    l: i32, r: i32, ac: i32, oc: i32, apples: &[i32], oranges: &[i32],
) {
    let count = |center: i32, deltas: &[i32]| {
        deltas
            .iter()
            .map(|delta| center + delta)
            .filter(|&pos| l <= pos && pos <= r)
            .count()
    };

    let apple_count = count(ac, apples);
    let orange_count = count(oc, oranges);

    println!("{}\n{}", apple_count, orange_count);
}
