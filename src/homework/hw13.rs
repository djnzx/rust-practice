/// Функція для обчислення зайнятої площі в двовимірному масиві
fn calculate_occupied_area(grid: &[Vec<u8>]) -> usize {
  grid.iter().flatten().filter(|&&cell| cell == 1).count()
}

fn main() {
  let grid = vec![
      vec![1, 0, 0, 1],
      vec![0, 1, 1, 0],
      vec![1, 1, 0, 0],
      vec![0, 0, 1, 1],
  ];

  let occupied_area = calculate_occupied_area(&grid);
  println!("Зайнята площа: {}", occupied_area);
}
