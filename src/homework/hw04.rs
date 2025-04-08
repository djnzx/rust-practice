const W: usize = 30;
const H: usize = 15;

fn main() {
    let mut result = String::new();

    for y in 0..H {
        for x in 0..W {
            if y == 0 || y == H - 1 {
                result.push('*'); // Верхній або нижній край
            } else if x == 0 || x == W - 1 {
                result.push('*'); // Лівий або правий край
            } else if y * (W - 1) == x * (H - 1) {
                result.push('\\'); // Діагональ зліва направо
            } else if y * (W - 1) == (W - 1 - x) * (H - 1) {
                result.push('/'); // Діагональ справа наліво
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    print!("{}", result);
}
