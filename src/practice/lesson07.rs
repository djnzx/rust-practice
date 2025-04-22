/// simple brackets_is_valid

fn is_valid(s: &str) -> bool {

    let mut count = 0;
    for c in s.chars() {
        match c { 
            '{' => count += 1,
            '}' if count == 0 => return false,
            '}' => count -= 1,
            _ => ()
        }
    }
    count == 0
}

#[test]
fn test() {
    let data = [
        ("", true),
        ("{}", true),
        ("{}{}", true),
        ("{{}{}}", true),
        ("{", false),
        ("}", false),
        ("{}}", false),
        ("{{}", false),
        ("}{", false),
    ];
    
    for (s, expected) in data {
        let result = is_valid(s);
        println!("checking: {s:10}, expected: {expected:5}, actual: {result:5}");
        assert_eq!(result, expected);
    }
    
}
