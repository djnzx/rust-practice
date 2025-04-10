const WIDTH: usize = 30;
const HEIGHT: usize = 13;

fn main() {
    let mut canvas = vec![vec![' '; WIDTH]; HEIGHT];

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            draw_border(&mut canvas, row, col);
            draw_diagonals(&mut canvas, row, col);
        }
    }

    for line in &canvas {
        println!("{}", line.iter().collect::<String>());
    }
}

fn draw_border(canvas: &mut Vec<Vec<char>>, y: usize, x: usize) {
    if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {
        canvas[y][x] = '*';
    }
}

fn draw_diagonals(canvas: &mut Vec<Vec<char>>, y: usize, x: usize) {
    let scaled_y = y as f32 / (HEIGHT - 1) as f32;
    let expected_x1 = (scaled_y * (WIDTH - 1) as f32).round() as usize;
    let expected_x2 = ((1.0 - scaled_y) * (WIDTH - 1) as f32).round() as usize;

    if x == expected_x1 || x == expected_x2 {
        canvas[y][x] = '*';
    }
}
