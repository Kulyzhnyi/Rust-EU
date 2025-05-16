fn invert_the_case(s: String) -> String {
    let mut result = String::new();

    for c in s.chars() {
        if c.is_lowercase() {
            result.push_str(&c.to_uppercase().to_string());
        } else if c.is_uppercase() {
            result.push_str(&c.to_lowercase().to_string());
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let data = [
        ("Hello", "hELLO"),
        ("Привет", "пРИВЕТ"),
    ];

    for i in 0..data.len() {
        let a = data[i].0;
        let b = data[i].1;

        let flipped = invert_the_case(a.to_string());
        println!("Вхід: {:<10} → Вихід: {}", a, flipped);
        assert_eq!(flipped, b.to_string());

        let flipped_back = invert_the_case(b.to_string());
        println!("Вхід: {:<10} → Вихід: {}", b, flipped_back);
        assert_eq!(flipped_back, a.to_string());
    }
}
