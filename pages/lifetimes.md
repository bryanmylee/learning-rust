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
