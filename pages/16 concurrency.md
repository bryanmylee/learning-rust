# Concurrency

_Concurrent programming_ is where different parts of a program execute independently, and _parallel programming_ is where different parts of a program execute at the same time.

In this note, when we use the term _concurrent_ programming, we are referring to both _concurrent_ and/or _parallel_ programming.

Many higher-level languages are dogmatic about the solutions offerred for concurrent problems, but Rust is a lower-level language that is expected to provide the solution with the best performance in any given situation. Therefore, Rust offers a variety of tools for modeling problems in whatever way is most appropriate.

We'll discuss:

- how to create threads to run multiple pieces of code at the same time
- _message-passing_ concurrency where channels send messages between threads
- _shared-state_ concurrency where multiple threads have access to some piece of data
- the `Sync` and `Send` traits, which extend Rust's concurrency guarantees to user-defined types and those provided by the standard library.

## Concurrency runtimes

In most modern systems, an executed program's code is run in a _process_, and the operating system manages multiple processes at once. Within the program, we can also have independent parts that run simultaneously. The features that run these independent parts are called _threads_.

Because there is no inherent guarantee about the order in which parts of our code on different threads run, threads often lead to problems such as:

- race conditions, where threads access resources in an inconsistent order
- deadlocks, where two threads wait for each other to finish using a resource indefinitely
- bugs that happen only in certain situations and are hard to reproduce and fix reliably

Rust attempts to mitigate the negative effects of using threads, but building a multithreaded application still takes careful thought and requires a code structure that is different from that in single-threaded applications.

_1:1 threads_ are threads that directly mapped to native threads exposed in the operating system APIs.

Many languages provide their own special implementations of threads known as _green threads_. These threads will be executed in the context of a different number of operating system threads, so the green-threaded model is sometimes called the _M:N_ model: there are _M_ green threads per _N_ operating system threads.

Each model has its advantages and disadvantages, but the most important trade-off to consider in Rust is runtime support.

In this context, _runtime_ refers to code that is included by the language in every binary. Any non-assembly language will have some runtime, albeit small. So colloquially when people say a language has "no runtime", they often mean "small runtime". Smaller runtimes have fewer features but have the advantage of resulting in smaller binaries. Rust takes the approach of having nearly no runtime size in exchange for more features. Rust needs to have nearly no runtime and cannot compromise on being able to call into C to maintain performance.

The green-threading _M:N_ model requires a larger language runtime to manage threads. As such, the Rust standard library only provides an implementation of 1:1 threading. Of course, there are crates like `tokio` that implement _M:N_ threading.

# Threads

```rs
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

## Waiting for threads to finish using `join` handles

In the code example above, there is no guarantee that a thread will run. We can fix this by saving the return value of `thread::spawn` in a variable. The return type is `JoinHandle`. A `JoinHandle` is an owned value that, when `join` is called on, will wait for its thread to finish.

```rs
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

Calling `join` blocks the thread currently running until the thread represented by the handle terminates. _Blocking_ a thread means that thread is prevented from performing work or exiting. If we had called `join` before the main thread got a chance to complete its work, it will be blocked from finishing that work.

## Using `move` closures with threads

The `move` closure is often used with `thread::spawn` to use data from one thread in another.

`thread::spawn` takes a closure that accepts no arguments, so to use data from the main thread in the spawned thread, the spawned thread's closure must capture the values it needs. Since the the spawned thread might outlive the main thread, we cannot guarantee that a reference will still be valid when needed. Therefore, we have to move ownership of the variables into the spawned thread.

# Message passing to transfer data between threads

_Message passing_ is a popular way to ensuring safe concurrency, where threads or **actors** communicate by sending each other messages containing data. A slogan from Golang's documentation: "Do not communicate by sharing memory; instead, share memory by communicating".

This can be implemented with _channels_, which the Rust standard library provides an implementation of. Channels have two halves: a transmitter and a receiver. The transmitter half is the upstream location where data is placed, and the receiver half is the downstream location where data is received.

A channel is said to be _closed_ if either half is dropped.

```rs
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}

```

`mpsc` stands for _multiple producer, single consumer_. Rust's standard library implements channels in a way that a channel can have multiple sending ends but only one receiving end.

We can move the transmitting end into a spawned thread and have it send one string.

```rs
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

If the channel is closed, `tx.send` will return `Err`.

`rx.recv` blocks the thread and waits until a value is received.

`rx.try_recv` does not block, but will return a `Result<T, E>` immediately: An `Ok` value holding a message if one is available and an `Err` value if there aren't any messages at the time.

# Shared state concurrency

_Mutexes_ only allow one thread to access some data at any given time. A thread must first signal that it wants access by asking to acquire the mutex's _lock_.

Mutexes are usually difficult to use because we have to remember two rules:

1. we must attempt to acquire the lock before using data
2. when we are done with the data, we must unlock the data so other threads can acquire

```rs
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
```

The `lock` call blocks the current thread until it is our turn to have the lock. If another thread panics while holding the lock, the call to `lock` will fail and no one else will be able to acquire the lock.

After we acquire the lock, we can treat the return value as a mutable reference to the value inside. The return value is type `MutexGuard<T>`, which is a smart pointer that has a `Drop` implementation to release the lock automatically.

## Sharing mutexes between threads

```rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Each thread needs to own the counter. To enable multiple ownership, we need reference counting. However, `Rc<T>` does not implement the `Send` trait. The `Send` trait indicates that a type can be safely sent between threads.

`Rc<T>` does not use concurrency-safe primitives to ensure that changes to its internal count can't be interrupted by another thread. Therefore, we need to use `Arc<T>` which has the same interface as `Rc<T>`, but it uses concurrent safe primitives. `Arc<T>` represents an _atomically reference counted_ type.

`Arc<T>` has more overheads than `Rc<T>` as it uses concurrent safe primitives that have more checks. If our code only runs on a single thread, we should simply use `Rc<T>`.

## Interior mutability of `Mutex<T>`

Although `Mutex<T>` is immutable, we are able to get a mutable reference to the value inside it with `lock` using interior mutability, much like the `Cell` family does.

Just like how we use `Mutex<T>` to mutate data inside an `Arc<T>`, we use `RefCell<T>` to mutate data inside an `Rc<T>`.

## Deadlocks

Similar to how `Rc<T>` cannot prevent us from making reference cycle mistakes, `Mutex<T>` cannot prevent us from creating _deadlocks_.

# `Send` trait

The `Send` marker trait indicates that an implementing type can have its ownership transferred between threads.

Almost every Rust type is `Send`, with a few exceptions like `Rc<T>`: this cannot be `Send` because if we clone the `Rc<T>` and transferred ownership to another thread, both threads might update the reference count at the same time.

Any type composed entirely of `Send` types is automatically marked as `Send` as well.

# `Sync` trait

The `Sync` marker trait indicates that it is safe for the implementing type to be referenced from multiple threads.

In other words, any type `T` is `Sync` if `&T` is `Send`, meaning the reference can be sent safely to another thread.

The smart pointer `Rc<T>` is not `Sync` for the same reasons it is not `Send`. The `Cell` family of types is also not `Sync` as the runtime implementation of borrow checking is not thread-safe.

Any type composed entirely of `Sync` is automatically marked as `Sync` as well.

# Implementing `Send` and `Sync` manually is unsafe

As marker traits, `Send` and `Sync` has **no methods to implement**. They are just useful for enforcing invariants related to concurrency. We can get `Send` and `Sync` by composing types entirely made up of `Send` and `Sync` traits.

Manually implementing these traits require `unsafe` Rust code.
