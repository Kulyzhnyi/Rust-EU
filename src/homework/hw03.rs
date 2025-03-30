fn main() {
    const W: u32 = 25; // ширина
    const H: u32 = 10; // висота
    let k = W as f32 / H as f32; // співвідношення ширини до висоти

    for y in 0..H {
        for x in 0..W {
            // побудова сторін
            let is_hor = y == 0 || y == H - 1;
            let is_ver = x == 0 || x == W - 1;

            // побудова діагоналей
            let is_first_diag = (x as f32 - k * y as f32).abs() < 0.5; // перша діагональ
            let is_second_diag = ((W - 1 - x) as f32 - k * y as f32).abs() < 0.5; // друга діагональ

            let sym = if is_hor || is_ver || is_first_diag || is_second_diag { '*' } else { ' ' };
            print!("{sym}");
        }
        println!();
    }
}