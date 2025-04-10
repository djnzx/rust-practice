use rand::Rng;

/// Підрахунок мінімальної кількості перенесень вантажу
fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        panic!("Неможливо рівномірно розподілити вантаж між кораблями");
    }

    let target = total / n;
    let mut moves = 0;

    for &ship in shipments {
        if ship > target {
            moves += (ship - target) as usize;
        }
    }

    moves
}

/// Генерація коректного вектора вантажів, що можуть бути розподілені рівномірно
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(10..50);
    let mut shipments = vec![target; n];

    // випадковим чином зменшимо кілька кораблів, а іншим додамо
    for i in 0..n / 2 {
        let delta = rng.gen_range(0..=target.min(10));
        shipments[i] -= delta;
        shipments[n - 1 - i] += delta;
    }

    shipments
}

fn main() {
    let shipments = gen_shipments(6);
    println!("Грузи по кораблях: {:?}", shipments);
    let moves = count_permutation(&shipments);
    println!("Мінімальна кількість переміщень: {}", moves);
}
