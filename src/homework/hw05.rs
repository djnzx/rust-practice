// hw05.rs

fn gcd(a: u32, b: u32) -> u32 {
  let mut a = a;
  let mut b = b;

  while b != 0 {
      let temp = b;
      b = a % b;
      a = temp;
  }

  a
}

fn main() {
  let x = 48;
  let y = 18;

  println!("GCD of {} and {} is {}", x, y, gcd(x, y));
}
