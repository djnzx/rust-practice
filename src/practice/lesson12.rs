use std::cmp::min;

/// parking_dilemma
fn roof_size(cars: &[i32], k: usize) -> usize {
    let mut cars = cars.to_vec();
    cars.sort();
    println!("{:?}", cars);
    let mut min_tent: usize = (cars.last().unwrap() - cars[0] + 1) as usize;
    for idx in 0..(cars.len() - k + 1) {
        let tent = (cars[idx + k - 1] - cars[idx] + 1) as usize;
        min_tent = min (tent, min_tent);
    }
    min_tent
}

// [6, 2, 12, 7]
//  0  1  2  3
//  2  6  7  12  15  16
//  -------
//        6
//     -------
//           7
//        ---------
//               9
//           ----------
//                    5
#[test]
fn test1() {
    let data = [6, 2, 12, 7, 15, 16];
    let k: usize = 3;
    let len = roof_size(&data, k);
    assert_eq!(len, 5);
}
