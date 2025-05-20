fn is_palindrome(x: u32) -> bool {
    let original = x.to_string();
    let chars: Vec<char> = original.chars().collect();
    let len = chars.len();

    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }

    true
}

fn main() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    data
        .iter()
        .for_each(|(n, exp)| {
            let result = is_palindrome(*n);
            println!("Число: {:<5} → Паліндром? {} (Очікувано: {})", n, result, exp);
            assert_eq!(result, *exp);
        });

    println!("Усі тести пройдено успішно.");
}
