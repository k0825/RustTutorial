fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y: 4.0 };

    let number_list = vec![34, 50, 20, 100, 65];
    let result = largest(&number_list);
    println!("{}", result);

    let char_list = vec!['1', 'a', 'd', 'z'];
    let result = largest(&char_list);
    println!("{}", result);
}
