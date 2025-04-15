fn is_valid(s: &str) -> bool {
    let mut c = 0;

    for ch in s.chars() {
        match ch {
            '{' => c += 1,
            '}' if c == 0 => return false,
            '}' => c -= 1,
            _ => {}
        }
    }

    c == 0
}

#[test]
fn is_valid_test() {
    let inputs = [
        ("{}", true),
        ("{}{}", true),
        ("{{}}", true),
        ("{{}{}}", true),
        ("{", false),

        ("{{}", false),
        ("{}}", false),
        ("}", false),
        ("}{", false),
    ];

    for (s, expected) in inputs {
        let actual = is_valid(s);
        println!("testing: {s:-10} expected: {expected:-5}, actual: {actual}");
        assert_eq!(actual, expected);
    }
}
