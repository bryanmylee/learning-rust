use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // We want the `Node` to own its children but also share that ownership with variables so each
    // `Node` can be accessed directly. To do this, we define the children to be `Vec<Rc<Node>>`.
    //
    // We also want to modify which nodes are children of other nodes, so we have a `RefCell<T>` on
    // `children`.
    children: RefCell<Vec<Rc<Node>>>,
    // To avoid reference cycles, we use `Weak<T>`. We don't have to use an `Option` here as
    // `Weak<T>` returns an optional when upgraded.
    //
    // We want to be able to modify the parent of a node, so we wrap the field in `RefCell<T>`.
    parent: RefCell<Weak<Node>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            // Because of `Rc::clone`, the node referenced by `leaf` now has two owners.
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        *(leaf.parent.borrow_mut()) = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
