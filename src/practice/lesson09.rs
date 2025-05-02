/// Some(x) - when a sequence is correct
/// None    - when a sequence is not correct
fn max_level(s: &str) -> Option<usize> {
    let mut max_level = 0;
    let mut current: isize = 0; // current depth
    
    for c in s.chars() {
        match c { 
            '(' => current += 1,
            ')' => current -= 1,
            _ => continue,
        }
        if current < 0 { break; }
        if current > max_level { max_level = current; }
    }
    if current == 0 { Some(max_level as usize) } else { None }
}

#[test]
fn test_max_level() {
    let test_data = [
        ("", Some(0_usize)),
        ("()", Some(1)),
        ("(a)", Some(1)),
        ("(b)(c)", Some(1)),
        ("(b)(c", None),
        ("(b))c", None),
        ("((a))", Some(2)),
        ("((x)(y))", Some(2)),
        ("(z((h)()))", Some(3)),
        ("(()((()7(((k)()()z)))))", Some(6)),
    ];

    test_data
        .iter()
        .for_each(|&(input, expected)| {
        let real = max_level(input);
        dbg!(
          "case: {:25} real:{} expected:{}",
          input, expected, real
        );
        assert_eq!(real, expected);
    })
}
