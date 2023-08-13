fn main() {
    vectors();
    strings();
    hashmaps();
}

fn vectors() {
    // type annotation required because empty initial value.
    let _empty_vector = Vec::<i32>::new();
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
    println!("The third element is {}", third);

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

fn strings() {
    // All types that implement `Display` will have a `to_string` method.
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    let mut prefix = String::from("foo");
    let suffix = String::from("bar");
    prefix.push_str(&suffix);
    // `push_str` takes an immutable reference to a string slice.

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 is moved here.
    // &s2 is of type `&String` which is automatically coerced into `&str` by turning `&s2` into
    // `&s2[..]` via deref coercion.
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);

    // For complex formatting, we can use the format macro.
    // It does not take ownership of any of its parameters.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    // Rust strings are UTF-8 encoded by default, and therefore cannot be integer indexed.
    // Instead, string slices must be used.
    let hello = "Здравствуйте";
    let slice = &hello[0..4];
    println!("slice is {}", slice);

    // We can also iterate over strings to get each Unicode scalar value with `.chars()`, but this
    // does not guarantee grouping of clusters.
    for c in "नमस्त".chars() {
        println!("{}", c);
    }
    // We can iterate over the bytes with `.bytes()`.
    for b in "नमस्त".bytes() {
        println!("{}", b);
    }
}

fn hashmaps() {
    use std::collections::HashMap;

    // HashMap will take ownership of the values it stores.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // type annotation needed to tell Rust what data type to collect into.
    // _ generic argument asks Rust to infer automatically.
    let scores = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect::<HashMap<_, _>>();

    // We can get values from a HashMap by providing its keys to the `get` method.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    if let Some(score) = score {
        println!("{} team has a score of {}", team_name, score)
    }

    // We can iterate over the entries of HashMap.
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // When updating a value in the HashMap, we have to handle the case where the key already
    // exists.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Overrides the value and returns an Option containing the previous value if it exists.
    let prev_value = scores.insert(String::from("Blue"), 20);
    if let Some(prev_value) = prev_value {
        println!("Blue team had a previous score of {}", prev_value);
    }

    // Insert if a previous value doesn't exist.
    // `entry(key)` returns an `Entry` enum that represents a value that might or might not exist
    // in the HashMap.
    let score = scores.entry(String::from("Blue")).or_insert(50);
    println!("Blue team has a score of {}", score);
    // Since score is a mutable reference to the value in the map entry, we have to dereference it
    // with `*` for read and write operations (if it is a mutable reference).
    *score += 5;
    println!("Blue team now has a score of {}", score);
    println!("scores: {:?}", scores);
}
