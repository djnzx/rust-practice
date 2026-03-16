const W: usize = 40;
const H: usize = 20;

fn main() {
    let mut output = String::new();

    for y in 0..H {
        for x in 0..W {
            if y == 0 || y == H - 1 {
                // Верхній або нижній край
                output.push('*');
            } else if x == 0 || x == W - 1 {
                // Лівий або правий край
                output.push('*');
            } else if x * H == y * W {
                // Діагональ зліва направо
                output.push('\\');
            } else if (W - 1 - x) * H == y * W {
                // Діагональ справа наліво
                output.push('/');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}

