fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut shift = n % len;
    if shift < 0 {
        shift += len;
    }

    for i in 0..len {
        let index = (i - shift + len) % len;
        result.push(chars[index as usize]);
    }

    result
}

fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)| {
            let result = rotate(s.clone(), *n);
            println!("Зсув: {n:>3} | Результат: {result} | Очікувано: {exp}");
            assert_eq!(result, *exp);
        });

    println!("Усі тести пройдено успішно.");
}
