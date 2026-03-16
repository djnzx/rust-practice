// hw08.rs

fn is_prime(n: u32) -> bool {
  if n <= 1 {
      return false;
  }
  if n == 2 {
      return true;
  }
  if n % 2 == 0 {
      return false;
  }
  let sqrt_n = (n as f64).sqrt() as u32;
  for i in (3..=sqrt_n).step_by(2) {
      if n % i == 0 {
          return false;
      }
  }
  true
}

fn main() {
  let num = 29;
  println!("Число {} є простим? -> {}", num, is_prime(num));
}
