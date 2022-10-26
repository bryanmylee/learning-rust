fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_item = &list[0];
    for item in list.iter() {
        if item > largest_item {
            largest_item = item
        }
    }
    largest_item
}

fn main() {
    let list = vec![1, 2, 3, 4, 5, 9, 10, 4, 5];
    println!("Largest in {:?} is {}", list, largest(&list));
}
