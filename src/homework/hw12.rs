use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let n = shipments.len() as u32;
    let total: u32 = shipments.iter().sum();

    if total % n != 0 {
        return -1;
    }

    let avg = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments {
        balance += weight as i32 - avg as i32;
        moves += balance.abs();
    }

    moves as i32 / 2
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(10..100);
    let mut shipments = vec![avg; n];

    for _ in 0..(n / 2) {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        let delta = rng.gen_range(1..=avg.min(10));
        if shipments[i] >= delta {
            shipments[i] -= delta;
            shipments[j] += delta;
        }
    }

    shipments
}

fn main() {
    let shipments1 = vec![1, 1, 1, 1, 6];
    let shipments2 = vec![9, 3, 7, 2, 9];
    let shipments3 = gen_shipments(10);

    println!("Example 1: {:?}", shipments1);
    println!("Moves: {}", count_permutation(&shipments1));

    println!("Example 2: {:?}", shipments2);
    println!("Moves: {}", count_permutation(&shipments2));

    println!("Generated: {:?}", shipments3);
    println!("Moves: {}", count_permutation(&shipments3));
}
