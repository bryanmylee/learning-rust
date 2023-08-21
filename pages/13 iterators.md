# Iterators

Iterators allow you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don't have to reimplement that logic yourself.

Iterators in Rust are _lazy_, meaning they have no effect until you call the methods that consume the iterator.

```rs
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

Iterators can be used with `for` loops.

```rs
for val in v1_iter {
    println!("Got: {val}");
}
```

## The `Iterator` trait and the `next` method

All iterators implement a trait `Iterator` that is defined in the standard library.

```rs
pub trait Iterator {
    type Item; // an associated type

    fn next(&mut self) -> Option<Self::Item>;
}
```

Implementing the `Iterator` trait requires that we also define an `Item` type, which is used in the return type of the `next` method.

```rs
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

Iterators need to be mutable, as calling the `next` method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. Each call to `next` _consumes_ the iterator.

The `iter` method produces an iterator over immutable references. If we want to create an iterator that takes ownership of the value and return owned values, we can call `into_iter` instead. If we want to create an iterator over mutable references, we can call `iter_mut`.

## Methods that consume the iterator

The `Iterator` trait has a number of different methods with default implementations provided by the standard library.

Methods that call `next` are referred to as _consuming adaptors_, as they use up the iterator. An example is `sum`.

```rs
let v1_iter = v1.iter();
let total: i32 = v1_iter.sum();
```

## Methods that produce other iterators

Methods that change iterators into different kinds of iterators are known as _iterator adaptors_. You can chain multiple calls to iterator adaptors to perform complex actions in a readable way.

But because all iterators are lazy, we have to call one of the consuming adaptor iterators to get results from calls to iterator adaptors.

```rs
// nothing happens.
v1.iter().map(|x| x + 1);

// something happens.
let v2 = v1.iter().map(|x| x + 1).collect::<Vec<_>>()
```
