#[test]
fn test_draw_envelope() {
    draw_envelope(30, 15);
}

fn draw_envelope(width: u8, height: u8) {
    for y in 0..height {
        for x in 0..width {
            let hor = y == 0 || y == height - 1;
            let ver = x == 0 || x == width - 1;
            let dia = x == y;
            
            let sym = if hor || ver || dia { '*' } else { ' ' };
            print!("{sym}");
        }
        println!();
    }
}
