
fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }
    let shift = ((n % len) + len) % len;

    let split_index = len - shift;
    let (left, right) = s.split_at(split_index as usize);

    format!("{}{}", right, left)
}
fn main() {
    let s = String::from("abcdefgh");
    let n = 2;

    let shifted = rotate(s.clone(), n);
    println!("Рядок: {}, Зсув на {} → {}", s, n, shifted);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        for (n, expected) in shifts.iter() {
            assert_eq!(rotate(s.clone(), *n), expected.to_string());
        }
    }
}
