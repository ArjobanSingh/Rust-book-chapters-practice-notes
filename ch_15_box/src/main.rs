mod list;
mod my_box;

use list::List::{Cons, Nil};
use my_box::MyBox;

fn main() {
    // box_to_store_i32();
    makeList();
    derefOp();

    // because deref will return &String, which can be converted to &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Manual deref for both string and str
    // Internally *m gives us String throught these steps:
    //   *(m.deref()) -> *(&String) -> String
    // Then &(String)[..] gives us -> &str;
    hello(&(*m)[..]);
}

// Box is nothing but a pointer stored on a Stack
// which points to a value stored on the heap.
fn box_to_store_i32() {
    let b = Box::new(14);
    println!("b = {b}");
}

// Recursive type with Box
// Const List (Lisp version of LinkedList)
// It contains pairs. 1. value, 2. Another nested pair
// Eg: (1, Pair2), Pair2 = (2, Pair3), Pair3 = (3, Nil)
// Combined: (1, (2, (3, Nil)))
fn makeList() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn derefOp() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("X, and Y is: {x} and {y}");

    // box ref
    let a = 14;
    // s is pointing to copied value of a in heap
    let s = MyBox::new(a);

    assert_eq!(14, a);
    assert_eq!(14, *s);

    let str = String::from("Arjoban");
    assert_eq!("Arjoban", &(*str));

    // Compiler won't allow moving from shared ref
    // let str_3 = *str_2;

    // THis move the ownership of string to box
    let str2 = Box::new(str);

}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
