/// brackets full

fn closing(c: char) -> bool {
    "])}>".contains(c)
}

fn corresponds(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') => true,
        ('{', '}') => true,
        ('[', ']') => true,
        ('<', '>') => true,
        _ => false,
    }
}

fn is_valid(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '[' | '(' | '{' | '<' => stack.push(c),
            c if closing(c) => {
                match stack.pop() {
                    Some(top) if corresponds(top, c) => (),
                    _ => return false
                }
            }
            _ => continue,
        }
    }
    stack.is_empty()
}

#[test]
fn test_is_valid() {
    let data = [
        ("", true),
        ("{}", true),
        ("{<>}", true),
        ("{<>[]}", true),
        ("{<>[()]}", true),
        ("{}{}", true),
        ("{{}}", true),
        ("{{}{}}", true),
        ("{<[>]}", false),
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
