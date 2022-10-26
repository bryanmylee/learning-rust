fn main() {
    // type annotation required because empty initial value.
    let _empty_vector: Vec<i32> = Vec::new();
    // macro shorthand
    let _vector = vec![1, 2, 3];

    // type is inferred from `push` calls.
    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    {
        let _v = vec![1, 2, 3, 4];
    } // _v is freed and all of its contents are also dropped.

    // Reading elements
    let mut v = vec![1, 2, 3, 4, 5];

    // If the value dues not exist, this will cause a panic.
    let third = &v[2];

    // This returns an Option type
    if let Some(third) = v.get(2) {
        println!("The third element is {}", third);
    } else {
        println!("There is no third element");
    }

    // Cannot borrow as immutable because an immutable reference `third` exists.
    // v.push(6);
    println!("The third element is: {}", third);

    // Vectors implement the Iterable trait.
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // Given a mutable reference to each element, we have to dereference it before modifying.
        *i += 50;
    }
}
