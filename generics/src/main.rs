fn main() {
    generic_methods();
}

pub trait Summary {
    // Specifying a trait method that has to be fulfilled by implementing structs.
    fn summarize_author(&self) -> String;

    // Default implementation for trait methods.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn generic_methods() {
    let list = vec![1, 2, 3, 4, 5, 9, 10, 4, 5];
    println!("Largest in {:?} is {}", list, largest(&list));

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(&string1, string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // Fails to compile because `result` will receive the lifetime of `string2` and be invalid
    // after the previous scope ends.
    // println!("The longest string is {}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_item = &list[0];
    for item in list.iter() {
        if item > largest_item {
            largest_item = item
        }
    }
    largest_item
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
