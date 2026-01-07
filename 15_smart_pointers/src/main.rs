use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

struct CustomSmartPointer {
        data: String,
}

impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data: {}", self.data);
        }
    }

/* 
enum List {
    Cons(i32, Box<List>),
    Nil,
}                     */

enum List {
    Cons(i32, Rc<List>),
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
    // let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
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

    //running code on cleanup with Drop trait
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");


    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");


    //Rc<T> for multiple ownership
    //sharing data between multiple parts of a program
   let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
   println!("Count after creating a = {}", Rc::strong_count(&a));
   let _b = Cons(3, Rc::clone(&a));
   println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(2, Rc::clone(&a));
        println!("Count after creating d = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));



}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

