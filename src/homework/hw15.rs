fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    // Перебираємо всі перестановки 8 цифр з 8
    for &m in &digits {
        for &u in &digits {
            if u == m { continue; }
            for &x in &digits {
                if x == m || x == u { continue; }
                for &a in &digits {
                    if a == m || a == u || a == x { continue; }
                    for &s in &digits {
                        if s == m || s == u || s == x || s == a { continue; }
                        for &l in &digits {
                            if [m, u, x, a, s].contains(&l) { continue; }
                            for &o in &digits {
                                if [m, u, x, a, s, l].contains(&o) { continue; }
                                for &n in &digits {
                                    if [m, u, x, a, s, l, o].contains(&n) { continue; }

                                    let muxa = 1000 * m + 100 * u + 10 * x + a;
                                    let slon = 1000 * s + 100 * l + 10 * o + n;

                                    if muxa * a == slon {
                                        count += 1;
                                        println!("  {}{}{}{}", m, u, x, a);
                                        println!("x    {}", a);
                                        println!("-------");
                                        println!(" {:04}", slon);
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

    println!("Варіантів рішень: {}", count);
}
