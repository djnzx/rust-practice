/// hackerrank intro: apple-and-orange
fn countApplesAndOranges1(l: i32, r: i32, ac: i32, oc: i32, apples: &[i32], oranges: &[i32]) {
    // count apples
    let mut apple_count = 0;
    for d in apples {
        let pos = ac + d;
        if pos >= l && pos <= r {
            apple_count += 1;
        }
    }

    // count oranges
    let mut orange_count = 0;
    for d in oranges {
        let pos = oc + d;
        if pos >= l && pos <= r {
            orange_count += 1;
        }
    }

    println!("{apple_count}\n{orange_count}")
}

fn count1(l: i32, r: i32, center: i32, fruits: &[i32]) -> i32 {
    let mut count = 0;
    for d in fruits {
        let pos = center + d;
        if pos >= l && pos <= r {
            count += 1;
        }
    }
    count
}

fn mk_absolute(center: i32, delta: i32) -> i32 {
    center + delta
}

fn count(l: i32, r: i32, center: i32, fruits: &[i32]) -> usize {
    fruits
        .iter()
        .map(|d| center + d)
        .filter(|&pos| pos >= l && pos <= r)
        .count()
}

fn countApplesAndOranges2(l: i32, r: i32, ac: i32, oc: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = count(l, r, ac, apples);
    let orange_count = count(l, r, oc, oranges);
    println!("{apple_count}\n{orange_count}")
}

fn countApplesAndOranges(l: i32, r: i32, ac: i32, oc: i32, apples: &[i32], oranges: &[i32]) {
    let count = |center: i32, fruits: &[i32]| 
        fruits.iter()
            .map(|d| center + d)
            .filter(|&pos| pos >= l && pos <= r)
            .count();

    let apple_count = count(ac, apples);
    let orange_count = count(oc, oranges);
    println!("{apple_count}\n{orange_count}")
}
