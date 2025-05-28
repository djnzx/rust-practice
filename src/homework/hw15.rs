fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    for &m in &digits {
        for &u in &digits {
            if u == m { continue; }
            for &x in &digits {
                if [m, u].contains(&x) { continue; }
                for &a in &digits {
                    if [m, u, x].contains(&a) { continue; }
                    let muha = m * 1000 + u * 100 + x * 10 + a;
                    for &s in &digits {
                        if [m, u, x, a].contains(&s) { continue; }
                        for &l in &digits {
                            if [m, u, x, a, s].contains(&l) { continue; }
                            for &o in &digits {
                                if [m, u, x, a, s, l].contains(&o) { continue; }
                                for &n in &digits {
                                    if [m, u, x, a, s, l, o].contains(&n) { continue; }

                                    let slon = s * 1000 + l * 100 + o * 10 + n;
                                    if muha * a == slon {
                                        count += 1;
                                        println!("  {}{}{}{}", m, u, x, a);
                                        println!("x     {}", a);
                                        println!("--------");
                                        println!("  {}{}{}{}", s, l, o, n);
                                        println!();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Знайдено розв’язків: {}", count);
}
