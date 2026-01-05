use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = Box::new(11);
    println!("b = {}", b);

    // Using Box to create a recursive data structure
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List created using Box smart pointers.");

    //treating small pointers like regular references
    //following the reference to the value
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    //using Box<T> like a regular reference
    let z = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *z);

    //defining our own smart pointer

    let a = 12;
    let my_box = MyBox(a);
    assert_eq!(12, a);
    assert_eq!(12, *my_box);
    println!("MyBox deref works correctly.");

    // using deref coercion in functions and methods
    let name = MyBox(String::from("Rust"));
    hello(&name);


}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
