/// console - envelope
fn envelope(width: u32, height: u32) {
    // 27 / 10 = > 2.7
    let k = width as f32 / height as f32;

    for y in 0..height {
        for x in 0..width {
            let is_hor = y == 0 || y == height - 1;
            let is_ver = x == 0 || x == width - 1;
            let is_diag = x == (y as f32 * k) as u32;
            let is_diag2 = x == width - 1 - (y as f32 * k) as u32;
            let show = is_hor || is_ver || is_diag || is_diag2;

            let c = if show { '*' } else { ' ' };
            print!("{}", c);
        }
        println!();
    }
}

#[test]
fn test_envelope() {
    envelope(10, 10);
    envelope(20, 10);
    envelope(23, 10);
    envelope(25, 10);
    envelope(27, 10);
    envelope(30, 10);
}

// x > y
// x >= y
// x < y
// x <= y
//
// x == y
// x != y
//
// (x == y)+-0.00001
// |x - y| < 0.00001
//
#[test]
fn test1() {
    let x = 0.1;
    let y = 0.2;
    let z = x + y;
    println!("{z}"); // 0.30000000000000004
}
