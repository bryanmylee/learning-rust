# Lifetimes

Every reference in Rust has a _lifetime_ which is **the scope for which that reference is valid**.

Most of the time, lifetimes are implicit and inferred. We must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure that the actual references used at runtime will definitely be valid.

## Preventing dangling references

The main aim of lifetimes is to prevent dangling references.

```rs
{
    let r;

    {
        let x = 5;
        r = &x;
    } // x is dropped and r becomes a dangling reference.
}
```

Lifetime annotations are required when the borrow checker cannot infer whether the reference returned will always be valid.

Lifetime annotations **do not change how long any of the references live**. Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

The names of lifetime parameters must start with a single quote `'` and are usually lowercase and very short.

```rs
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation does not have much meaning because they are meant to inform Rust how generic lifetime parameters of muliple references relate to each other.

For example, we can enforce that two arguments to a function have lifetimes as long as the generic lifetime.

```rs
// For some lifetime `'a`, the function takes two parameters, both of which are
// string slices that live at least as long as `'a`.
// The returned slice will live at least as long as lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // --snip--
}
```

Rust can analyze code within the function without any help. However, when a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own.

When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the overlapping scope of `x` and `y`. In other words, it takes the intersection of the lifetimes of `x` and `y`. Because we've annotated the lifecycle of the return value, the returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`.

## Lifetime annotations in struct definitions

It is possible for structs to hold references, but in that case we would need to add lifetime annotations on every reference in the struct's definition.

```rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

This annotation means that `i` cannot outlive the reference it holds in its `part` field. Therefore, `first_sentence` cannot go out of scope until `i` goes out of scope. Similarly, because `first_sentence` is a slice of `novel`, `novel` cannot go out of scope until `i` goes out of scope.

## Lifetime elision

In some common cases, Rust can infer the lifetime annonations of functions based on predictable and deterministic patterns called _lifetime elision rules_. These are a set of particular cases that the compiler will consider, and if the code fits these cases, we do not have to write lifetimes explicitly.

```rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

Lifetimes on functions or method parameters are called _input lifetimes_ and lifetimes on return values are called _output lifetimes_.

There are three lifetime elision rules considered and applied in sequence. These rules apply to `fn` as well as `impl` blocks. If the compiler reaches the end without completing lifetime elision, it will stop with an error.

The first rule is that each parameter that is a reference gets its own lifetime parameter.

```rs
fn foo(x: &i32);
// becomes
fn foo<'a>(x: &'a i32);

fn foo(x: &i32, y: &i32);
// becomes
fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
```

The second rule is that if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.

```rs
fn foo<'a>(x: &'a i32) -> &i32;
// becomes
fn foo<'a>(x: &'a i32) -> &'a i32;

fn foo<'a>(x: &'a i32) -> (&i32, &i32);
// becomes
fn foo<'a>(x: &'a i32) -> (&'a i32, &'a i32);
```

The third rule is if there are multiple input parameters but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.

```rs
impl<'a> Article<'a> {
    // first rule applied to add lifetime annotation to `content`.
    fn write<'b>(&self, &'b str content) -> (&i32, &str);
    // becomes
    fn write<'b>(&self, &'b str content) -> (&'a i32, &'a str);
}
```

## The static lifetime

One special lifetime is `'static`, which denotes the entire duration of the program. All string literals have the `'static` lifetime which we can annotate as:

```rs
let s: &'static str = "I have a static lifetime.";
```
