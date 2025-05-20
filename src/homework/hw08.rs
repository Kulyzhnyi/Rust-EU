fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }

    for i in 2..=(*n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    test_data.iter().for_each(|(n, expected)| {
        let result = is_prime(n);
        println!("Testing is_prime({}) → {}, expected: {}", n, result, expected);
        assert_eq!(result, *expected);
    });
}
