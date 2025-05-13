use std::cmp::{max, min};

/// parking_dilemma
fn size_of_any2(l: usize, r: usize) -> usize {
    r - l + 1
}

fn size_of_any(ns: &[u32]) -> u32 {
    let mut ns = ns.to_vec();
    ns.sort();

    ns.last().unwrap() - ns.first().unwrap() + 1
}

fn size3(n1: u32, n2: u32, n3: u32) -> u32 {
    let mn = min(min(n1, n2), n3);
    let mx = max(max(n1, n2), n3);
    mx - mn + 1
}

#[test]
fn test_size3() {
    let data = [
        ((1, 2, 3), 3),
        ((1, 2, 4), 4),
        ((1, 2, 10), 10),
        ((10, 15, 20), 11),
        ((51, 52, 53), 3),
        ((2, 6, 7), 6),
        ((6, 7, 12), 7),
        ((100, 150, 200), 101),
        ((150, 100, 200), 101),
        ((200, 100, 150), 101),
    ];

    for ((n1, n2, n3), expected) in data {
        let real = size3(n1, n2, n3);
        assert_eq!(real, expected);
    }

    assert_eq!(size_of_any(&[200, 100, 150]), 101);
}

fn parking_roof1(cars: &[usize], k: usize) -> usize {
    let mut min_size = usize::MAX;

    let mut cars = cars.to_vec();
    cars.sort();

    let car_at = |idx: usize| cars[idx];

    for idx in 0..=(cars.len() - k) {
        let idx_r = idx + (k - 1);
        let size = size_of_any2(car_at(idx), car_at(idx_r));
        min_size = min(size, min_size);
    }

    min_size
}

fn parking_roof(cars: &[usize], k: usize) -> usize {
    let mut cars = cars.to_vec();
    cars.sort();
    let car_at = |idx: usize| cars[idx];

    let it_l = 0..=(cars.len() - k);
    let it_r = (k - 1)..;

    it_l.zip(it_r)
        .map(|(l, r)| car_at(r) - car_at(l) + 1)
        .min()
        .unwrap()
}

#[test]
fn test1() {
    let data = [
        (vec![2, 6, 7, 12], 6),
        (vec![2, 6, 101, 103, 105, 7, 12], 5),
        (vec![2, 6, 1001, 101, 1002, 1003, 103, 105, 7, 12], 3),
    ];

    for (cars, expected) in data {
        let real = parking_roof(&cars, 3);
        assert_eq!(real, expected);
    }
}
