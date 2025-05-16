use std::collections::HashSet;

/// brackets full
fn matches(open: char, close: char) -> bool {
    match (open, close) {
        ('{', '}') => true,
        ('(', ')') => true,
        ('[', ']') => true,
        ('<', '>') => true,
        _ => false,
    }
}

fn is_valid(s: &str) -> bool {
    let open = HashSet::from(['{', '[', '(', '<']);
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            c if open.contains(&c) => stack.push(c),
            c => match stack.pop() {
                Some(open) => if !matches(open, c) { return false; }
                None => return false,
            }
        }
    }

    stack.is_empty()
}

#[test]
fn test1() {
    let data = [
        ("", true),
        ("{}", true),
        ("{<>}", true),
        ("{<>[]}", true),
        ("{<>()[]}", true),
        ("{<[()]>(<>)[()]}", true),
        ("{", false),
        ("}", false),
        ("{<[>]}", false),
        ("{}}", false),
        ("{{}", false),
        ("}{", false),
    ];

    for (s, expected) in data {
        let result = is_valid(s);
        println!("checking: {s:20}, expected: {expected:5}, actual: {result:5}");
        assert_eq!(result, expected);
    }
}
