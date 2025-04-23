// use crate::List::{Cons, Nil};
//
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'", self.data);
    }
}

fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let u = 5;
    let v = MyBox::new(u);

    assert_eq!(5, u);
    assert_eq!(5, *(v.deref()));

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };


    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
    drop(c);
}
