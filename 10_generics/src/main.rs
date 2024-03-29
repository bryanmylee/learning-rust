use std::fmt::{Debug, Display};

fn main() {
    trait_bounds();
    generic_methods();
    lifetimes();
}

fn trait_bounds() {
    let tweet = Tweet {
        username: String::from("bryanleebmy"),
        content: String::from("We are watching Shameless!"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
    notify(tweet);
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/**
 * The trait bound syntax is required if we want to specify more complex requirements e.g.
 * constraining two parameters to have the same type.
 */
pub fn notify_trait_bound_syntax<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

/**
 * Use + to specify multiple trait bounds.
 */
pub fn multiple_bounds<T: Summary + Display>(item: T) {
    println!("{}", item); // {} can be used to format due to `Display` trait.
}

/**
 * Too many generic trait bounds can be unreadable as a lot of trait bound information is placed
 * between the function name and its parameters, making the signature hard to read.
 *
 * We can use the `where` clause to specify trait bounds after the parameter list.
 */
pub fn where_clause<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

/**
 * We can also use the `impl Trait` syntax to return a value of some type that implements some
 * trait. This is especially useful for closures and iterators as we can return some type that
 * implements the `Iterator` trait without writing out the whole type.
 *
 * However, **this does not allow for polymorphic return types**. We cannot return a union of
 * multiple types because the size, alignment, and other properties of the type would not match.
 *
 * `impl Trait` return syntax is purely a cosmetic improvement.
 */

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

/**
 * We can conditionally implement methods for types that satisfy a given trait bound.
 */
struct Pair<T> {
    x: T,
    y: T,
}

/**
 * Create a generic implementation for any type `T` in `Pair<T>`.
 */
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/**
 * Create implementations for `Pair<T>` where `T` is `Display + PartialOrd`.
 */
impl<T> Pair<T>
where
    T: Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x {} is larger than y", self.x);
        }
    }
}

/**
 * Blanket implementations for traits allow us to automatically implement traits for any type that
 * satisfy another set of traits.
 *
 * Note that we can only create blanket implementations for local traits.
 */
impl<T> Summary for T
where
    T: Display,
{
    fn summarize_author(&self) -> String {
        self.to_string()
    }
}

fn generic_methods() {
    let list = vec![1, 2, 3, 4, 5, 9, 10, 4, 5];
    println!("Largest in {:?} is {}", list, largest(&list));
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

fn lifetimes() {
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

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let i2: ImportantExcerpt;
    {
        let s2 = String::from("A new novel");
        i2 = ImportantExcerpt { part: &s2 };
        // i2 is valid here...
        println!("Excerpt part in s2's scope: {}", i2.part);
    }
    // but not here.
    // println!("Excerpt part outside s2's scope: {}", i2.part);

    //`'static` lifetimes last the entire duration of the program's execution.
    // All string literals exist in the program's binary and therefore will always be available.
    let _s: &'static str = "A static lifetime";

    // Even though lifetime parameters are specified the generic arguments, they are omitted when
    // specifying specific types.
    longest_with_an_announcement::<String>("hi", "bye", String::from("an announcement"));
}

/**
 * Annotating a parameter with `'a` implies that the parameter will have a lifetime as long as `'a`.
 *
 * By annotating both `x` and `y` with the same lifetime parameter, we tell Rust that both `x` and
 * `y` lives at least as long as `'a`. `'a` therefore cannot live longer than either `x` or `y`,
 * and will represent a lifetime that is the overlap of `x` and `y`'s lifetime.
 *
 * This tells Rust that the returned reference will have a lifetime that lasts as long as both `x`
 * and `y`.
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
