use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    data.windows(2)
        .enumerate()
        .map(|(i, w)| (i, w[0] + w[1]))
        .min_by_key(|&(_, sum)| sum)
        .unwrap()
}

fn print_vector_with_highlight(data: &[i32], min_index: usize) {
    println!("Згенерований вектор:");
    for (i, &val) in data.iter().enumerate() {
        if i == min_index {
            print!("[{} ", val);
        } else if i == min_index + 1 {
            print!("{}] ", val);
        } else {
            print!("{} ", val);
        }
    }
    println!();
}

fn main() {
    let vec = gen_random_vector(20);
    let (min_index, min_sum) = min_adjacent_sum(&vec);
    print_vector_with_highlight(&vec, min_index);
    println!("Мінімальна сума пари: {} (позиції {} і {})", min_sum, min_index, min_index + 1);
}
