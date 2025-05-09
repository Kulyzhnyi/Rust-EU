const HEIGHT: usize = 11;

fn main() {
    let mut output = String::new();
    let mid = HEIGHT / 2;
    

    // Верхня частина (включно з центром)
    for i in 0..=mid {
        let spaces = mid - i;
        let stars = 2 * i + 1;
        output += &" ".repeat(spaces);
        output += &"*".repeat(stars);
        output += "\n";
    }

    // Нижня частина
    for i in (0..mid).rev() {
        let spaces = mid - i;
        let stars = 2 * i + 1;
        output += &" ".repeat(spaces);
        output += &"*".repeat(stars);
        output += "\n";
    }

    print!("{}", output);
}
