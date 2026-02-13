use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }
    (min_index, min_sum)
}

fn print_with_highlight(data: &[i32], min_index: usize) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:    ");
    for &val in data {
        print!("{:>3} ", val);
    }
    println!();

    print!("indexes: ");
    for i in 0..data.len() {
        if i == min_index {
            print!("\\__ ");
        } else if i == min_index + 1 {
            print!("__/ ");
        } else {
            print!("     ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        data[min_index] + data[min_index + 1],
        min_index,
        min_index + 1
    );
}

fn main() {
    let data = gen_random_vector(20);
    let (min_index, _) = min_adjacent_sum(&data);
    print_with_highlight(&data, min_index);
}
