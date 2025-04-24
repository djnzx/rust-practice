fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }

    for i in 2..=(*n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let numbers = [0, 1, 2, 3, 4, 5, 100, 10007];

    for num in numbers.iter() {
        println!("{} is prime? {}", num, is_prime(num));
    }
}

#[test]
fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    test_data.iter().for_each(|(n, prime)| {
        assert_eq!(is_prime(n), *prime)
    });
}
