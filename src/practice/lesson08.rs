/// simple brackets_is_valid
fn is_valid1(s: &str) -> bool {
    let mut l = 0;
    let mut r = 0;

    for c in s.chars() {
        if c == '{' {
            l += 1;
        } else if c == '}' {
            r += 1;
        }

        if r > l {
            break;
        }
    }

    l == r
}

fn is_valid2(s: &str) -> bool {
    let mut c = 0;

    for ch in s.chars() {
        if ch == '{' {
            c += 1;
        } else if ch == '}' {
            c -= 1;
        }

        if c < 0 {
            break;
        }
    }

    c == 0
}

fn is_valid3(s: &str) -> bool {
    let mut c = 0;
    for ch in s.chars() {
        match ch {
            '{' => c += 1,
            '}' => c -= 1,
            _ => continue,
        }
        if c < 0 {
            break;
        }
    }
    c == 0
}

fn is_valid(s: &str) -> bool {
    let mut c = 0;
    for ch in s.chars() {
        match ch {
            '{' => c += 1,
            '}' if c == 0 => return false,
            '}' => c -= 1,
            _ => continue,
        }
    }
    c == 0
}

#[test]
fn test_is_valid() {
    let data = [
        ("", true),
        ("{}", true),
        ("{}{}", true),
        ("{{}}", true),
        ("{{}{}}", true),
        ("{", false),
        ("}", false),
        ("{}{", false),
        ("{}}", false),
        ("}{", false),
    ];
    for (s, expected) in data {
        let result = is_valid(s);
        println!("checking: {s:10}, expected: {expected:5}, real: {result}");
        assert_eq!(result, expected);
    }
}
