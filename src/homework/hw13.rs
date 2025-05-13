fn roof_size(l: usize, r: usize) -> usize {
    r - l + 1
}

fn min_roof(cars: &[usize], k: usize) -> usize {
    let mut cars = cars.to_vec();
    cars.sort();
    let mut min_size = usize::MAX;

    for idx_l in 0..=(cars.len() - k) {
        let idx_r = idx_l + (k - 1);
        let pos_l = cars[idx_l];
        let pos_r = cars[idx_r];
        let size = roof_size(pos_l, pos_r);
        if size < min_size { min_size = size; }
    }

    min_size
}

#[test]
fn test_roof_size() {
    let data = [([2, 7], 6), ([51, 53], 3), ([101, 105], 5)];

    for (cars, expected) in data {
        let [l, r] = cars;
        let real = roof_size(l, r);
        assert_eq!(real, expected);
    }
}

#[test]
fn test_min_roof() {
    let data = [
        ([2, 12, 6, 7], 6),
        ([1, 53, 51, 52], 3),
        ([101, 53, 103, 105], 5),
    ];

    for (cars, expected) in data {
        let real = min_roof(&cars, 3);
        assert_eq!(real, expected);
    }
}
