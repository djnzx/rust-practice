/// hackerrank intro: apple-and-orange

fn countApplesAndOranges(house_l: i32, house_r: i32,
    apple_center: i32, orange_center: i32,
    apples: &[i32], oranges: &[i32],
) {
    let count = |center: i32, deltas: &[i32]| {
        deltas
            .iter()
            .map(|d| center + d)
            .filter(|&pos| house_l <= pos && pos <= house_r)
            .count()
    };

    let apple_count = count(apple_center, apples);
    let orange_count = count(orange_center, oranges);

    println!("{apple_count}\n{orange_count}");
}
