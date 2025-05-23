fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        vec![String::from("")]
    } else {
        let prev = gray(n - 1);
        let mut res = Vec::new();
        for x in &prev {
            res.push(format!("0{}", x));
        }
        for x in prev.iter().rev() {
            res.push(format!("1{}", x));
        }
        res
    }
}

#[test]
fn test() {
    let test_data = [
        (0, vec!["".to_string()]),
        (1, vec!["0".to_string(), "1".to_string()]),
        (2, vec!["00", "01", "11", "10"]
            .iter().map(|s| s.to_string()).collect()),
        (3, vec!["000", "001", "011", "010", "110", "111", "101", "100"]
            .iter().map(|s| s.to_string()).collect()),
        (4, vec!["0000", "0001", "0011", "0010",
                 "0110", "0111", "0101", "0100",
                 "1100", "1101", "1111", "1110",
                 "1010", "1011", "1001", "1000"]
            .iter().map(|s| s.to_string()).collect()),
    ];

    for (n, expected) in test_data {
        assert_eq!(gray(n), expected);
    }
}

fn main() {
    for i in 0..=4 {
        let g = gray(i);
        println!("gray({}): {:?}", i, g);
    }
}
