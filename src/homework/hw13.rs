struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 6 } },
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
    ]
}

fn area_occupied(rects: &Vec<Rectangle>) -> i32 {
    let mut cells: Vec<(i32, i32)> = Vec::new();

    for rect in rects {
        let left = rect.a.x.min(rect.b.x);
        let right = rect.a.x.max(rect.b.x);
        let top = rect.a.y.max(rect.b.y);
        let bottom = rect.a.y.min(rect.b.y);

        for x in left..right {
            for y in bottom..top {
                let point = (x, y);
                if !cells.contains(&point) {
                    cells.push(point);
                }
            }
        }
    }

    cells.len() as i32
}

fn main() {
    let data = test_data();
    let result = area_occupied(&data);
    println!("Зайнята площа: {}", result);
    assert_eq!(result, 60);
}
