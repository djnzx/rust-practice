#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

use std::collections::HashSet;

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();

    for rect in xs {
        let x1 = rect.a.x.min(rect.b.x);
        let x2 = rect.a.x.max(rect.b.x);
        let y1 = rect.a.y.min(rect.b.y);
        let y2 = rect.a.y.max(rect.b.y);

        for x in x1..x2 {
            for y in y1..y2 {
                occupied.insert((x, y));
            }
        }
    }

    occupied.len() as i32
}

fn area_occupied_test() {
    let data = test_data();
    let result = area_occupied(&data);
    assert_eq!(result, 60);
    println!("Зайнята площа: {}", result);
}

fn main() {
    area_occupied_test();
}
