use rand::Rng;


fn gen_random_vector(n: usize) -> Vec<i32> {  // Генерує вектор довжиною n зі значеннями від 10 до 99
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..n {
        let number = rng.gen_range(10..=99);
        vec.push(number);
    }
    vec
}


fn min_adjacent_sum(data: &Vec<i32>) -> (usize, i32, i32) {  // Знаходить індекс пари з мінімальною сумою
    let mut min_sum = data[0] + data[1];
    let mut index = 0;
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            index = i;
        }
    }
    (index, data[index], data[index + 1])
}


fn print_vector_with_min(data: &Vec<i32>) {   // Виводить усе красиво
    // Виводимо індекси
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>4}.", i);
    }
    println!();

    // Виводимо дані
    print!("data:  [");
    for i in 0..data.len() {
        if i != 0 {
            print!(", ");
        }
        print!("{:>2}", data[i]);
    }
    println!("]");

    
    let (min_index, a, b) = min_adjacent_sum(data);  // Малюємо стрілочку до мінімальної пари

    print!("indexes:");
    for i in 0..data.len() {
        if i == min_index {
            print!("{:>4}", "\\__");
        } else if i == min_index + 1 {
            print!("{:>4}", "__/");
        } else {
            print!("{:>4}", "");
        }
    }
    println!();

    
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        a,
        b,
        a + b,
        min_index,
        min_index + 1
    );
}

fn main() {
    let numbers = gen_random_vector(20);
    print_vector_with_min(&numbers);
}
