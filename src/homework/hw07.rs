fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn main() {
    let input = String::from("Hello, Привет!");
    let result = invert_the_case(input.clone());
    println!("Original: {}", input);
    println!("Inverted: {}", result);
}

#[test]
fn test() {
    let data = [
        ("Hello", "hELLO"),
        ("Привет", "пРИВЕТ"),
    ];

    data.iter().for_each(|(a, b)| {
        assert_eq!(invert_the_case(a.to_string()), b.to_string());
        assert_eq!(invert_the_case(b.to_string()), a.to_string());
    });
}
