const HEIGHT: usize = 6;

fn main() {
    let mut romb = String::new();

    for row in 0..HEIGHT {
        let spaces = HEIGHT - row - 1;
        let stars = row * 2 + 1;

        for _ in 0..spaces {
            romb.push(' ');
        }

        for _ in 0..stars {
            romb.push('*');
        }

        romb.push('\n');
    }
    for row in (0..HEIGHT - 1).rev() {
        let spaces = HEIGHT - row - 1;
        let stars = row * 2 + 1;

        for _ in 0..spaces {
            romb.push(' ');
        }

        for _ in 0..stars {
            romb.push('*');
        }

        romb.push('\n');
    }

    print!("{}", romb);
}
