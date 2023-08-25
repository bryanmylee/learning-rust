# Polymorphism

Although most people conflate polymorphism with inheritance, they are two separate concepts.

Inheritance is usually not the right solution as it:

1. often does not model problems properly,
2. shares too much detail from parent classes with child classes,
3. reduces flexibility in the code design, especially in languages that only allow single inheritance.

Rust does not implement inheritance, instead choosing to use traits, trait bounds, and default implementations for code re-use.

On the other hand, polymorphism is quite useful, allowing the substitution of multiple objects at runtime with one another if they share certain characteristics. Rust uses generics to abstract over the possible types and trait bounds to impose constraints on what those types must provide. This is called **bounded parametric polymorphism**.

## Polymorphism vs enums

We previously used enums to allow multiple types to be stored.

```rs
enum SpreadsheetCell {
  Float(f64),
  Integer(i32),
  Text(String),
}
```

However, the issue with enums is that they are _closed_ and only allow predefined types to be stored. If we want to allow extensibility on our type, we need polymorphism.

## Traits for common behavior

We'll define a trait to describe all the common behavior we expect our extensible type to have.

```rs
pub trait Draw {
    fn draw(&self);
}
```

A **trait object** points to both _an instance of a type implementing the trait_ as well as _a table used to look up trait methods on that type at runtime_. We create trait objects by specifying some sort of indirection on the trait, using `&` or `Box<T>`, and the `dyn` keyword.

We can use trait objects in place of a generic or concrete type. Whenever we use trait objects, Rust's type system ensures at compile time that any value used in that context will implement the trait object's trait.

We avoid calling structs and enums "objects" to distinguish them from other languages' objects. In a struct or enum, data in the fields and behavior in `impl` blocks are separated, whereas in other languages, data and behavior are combined into a single concept called an object.

However, trait objects are more similar to objects in that they combine data and behavior. The difference is that we cannot add data to a trait object. Their specific purpose is to allow abstraction across common behavior.

To define a trait object, we use the `dyn` keyword. This tells Rust that the object in question is a stand-in for any type that implements our trait, in this case `Draw`.

```rs
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

We can then define methods that use the trait object.

```rs
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

This works differently than using a generic type parameter with trait bounds.

```rs
impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

This implementation only works if `components` contains a homogeneous collection, and will be monomorphized at compile time to use concrete types.

This concept of being only concerned only with the messages a value responds to rather than the value's concrete type is similar to the concept of _duck typing_: if it walks and quacks like a duck, then it must be a duck!

`Screen::run` won't check whether each component is an instance of `Button` or `SelectBox`, it just calls the `draw` method on the component because we've specified the type to be a trait object `Box<dyn Draw>`.

## Trait objects use dynamic dispatch

Generics are monomorphized so that function calls can be **statically dispatched**, giving us the flexibility of generics without the performance cost of them. However, trait objects cannot be statically compiled exactly because the concrete types are not known at compile time.

Instead, Rust must use **dynamic dispatch** with trait objects. At runtime, Rust uses the pointers inside the trait object to find the method to call. There is a runtime cost when this lookup happens that does not occur with static dispatch. Furthermore, dynamic dispatch prevents the compiler from inlining a method's code which in turn prevents more optimizations.

This is a fundamental limitation of polymorphism that cannot be avoided. However, the flexibility is often worth it, so this is a trade-off to consider.

## Object safety is required for trait objects

You can only make _object-safe_ traits into trait objects. There are complex rules that make a trait object safe, but in practice, only two rules are relevant.

A trait is object-safe if all the methods defined in the trait:

1. do not return a type of `Self`
2. do not have generic type parameters.

Trait objects must be object safe because once we've used a trait object, Rust no longer knows the concrete type that's implementing that trait. If the trait method returns the concrete `Self` type, but the trait object forgets what `Self` exactly is, there is no way the method can use the original concrete type.

The same is true for generic type parameters that are filled in with concrete type parameters when the trait is used: the concrete types become part of the type that implements the trait. When the type is forgotten through the use of trait objects, there is no way to know what types to fill in the generic parameters with.
