fn min_moves(xs: &Vec<i32>) -> i32 {
    let n = xs.len() as i32;
    let mut total = 0;

    // рахуємо суму
    for i in 0..xs.len() {
        total += xs[i];
    }

    // якщо не ділиться порівну повертаємо -1
    if total % n != 0 {
        return -1;
    }

    let avg = total / n;
    let mut moves = 0;

    // рахуємо, скільки треба перемістити
    for i in 0..xs.len() {
        if xs[i] > avg {
            moves += xs[i] - avg;
        }
    }

    moves
}

// тестуємо і виводимо результати
fn main() {
    let a = vec![1, 1, 1, 1, 6];
    println!("moves for {:?} => {}", a, min_moves(&a));

    let b = vec![9, 3, 7, 2, 9];
    println!("moves for {:?} => {}", b, min_moves(&b));

    let c = vec![1, 2, 3];
    println!("moves for {:?} => {}", c, min_moves(&c));

    let d = vec![5, 5, 5, 5];
    println!("moves for {:?} => {}", d, min_moves(&d));

    let e = vec![1, 2, 4]; // неможливо
    println!("moves for {:?} => {}", e, min_moves(&e));
}
