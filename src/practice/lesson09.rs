/// shoes pair
fn shoes_group(s: String) -> usize {
    let mut level = 0;
    let mut count = 0;
    for c in s.chars() {
        match c {
            'L' => level -= 1,
            'R' => level += 1,
            _ => continue,
        }
        if level == 0 {
            count += 1;
        }
    }
    count
}

#[test]
fn test_shoes_group() {
    let test_data = [
        ("", 0),
        ("RL", 1),
        ("LR", 1),
        ("LRLR", 2),
        ("LLRLRR", 1),
        ("LLRLRLRLRLRLRR", 1),
        ("RLRRLLRLRRLL", 4),
        ("RLLLRRRLLR", 4),
    ];

    for (input, expected) in test_data {
        let real = shoes_group(input.to_string());
        println!("case: {input:20} expected: {expected} real: {real}");
        assert_eq!(real, expected);
    }
}
