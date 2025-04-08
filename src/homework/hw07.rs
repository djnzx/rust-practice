// hw07.rs

fn swap_case(input: &str) -> String {
  input
      .chars()
      .map(|c| {
          if c.is_lowercase() {
              c.to_ascii_uppercase()
          } else if c.is_uppercase() {
              c.to_ascii_lowercase()
          } else {
              c
          }
      })
      .collect()
}

fn main() {
  let text = "ПрИвІт, WoRlD!";
  let swapped = swap_case(text);
  println!("Original: {}", text);
  println!("Swapped: {}", swapped);
}
