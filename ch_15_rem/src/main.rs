use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use List::Cons;
use List::Nil;

impl List {
    fn get_all_items_in_vec(&self) -> Vec<i32> {
        let mut res = vec![];
        let mut current = self;
        while let List::Cons(val, next) = current {
            res.push(*val.borrow());
            current = next;
        };

        return res;
    }
}

fn main() {
    // multi_ownership();
    multi_and_mut_ownership();
}

// fn multi_ownership() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }


// multi owernship + mutability
fn multi_and_mut_ownership() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
// UNSAFE RUST: Interior mutability pattern

// Enforcing Borrowing Rules at Runtime with RefCell<T> in lib.rs