/// shoes pair
fn count_groups(s: &str) -> i32 {
    let mut balance = 0;
    let mut count = 0;

    for c in s.chars() {
        match c {
            'R' => balance += 1,
            'L' => balance -= 1,
            _ => continue,
        };
        if balance == 0 {
            count += 1;
        }
    }

    count
}

#[test]
fn test() {
    let test_data = [
        ("LR", 1),
        ("LLRR", 1),
        ("LRLR", 2),
        ("LRRL", 2),
        ("RLRRLLRLRRLL", 4),
        ("RLLLRRRLLR", 4),
        ("LLRLRLRLRLRLRR", 1),
    ];

    for (input, expected) in test_data {
        let real = count_groups(input);
        println!("input: {input:20} expected: {expected} real {real}");
        assert_eq!(real, expected);
    }
}

fn test2() {
    let mut count = 0;
    let x = count += 1; // statement:  x = (); count = count + 1
    let x: i32 = count + 1; // expression: x =     count + 1
}
