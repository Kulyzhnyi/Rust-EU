fn main() {
    let triangle_count = 6;
    let mut output = String::new();

    for t in 1..=triangle_count {
        for i in 0..t {
            let spaces = triangle_count - 1 - i;
            let stars = 2 * i + 1;
            output += &" ".repeat(spaces);
            output += &"*".repeat(stars);
            output += "\n";
        }
    }

    print!("{}", output);
}
