#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

// Note: Functions can implement all three of the Fn traits too. If what we want to do
// doesn’t require capturing a value from the environment, we can use the name of a
// function rather than a closure where we need something that implements one of the Fn
// traits. For example, on an Option<Vec<T>> value, we could call
// unwrap_or_else(Vec::new) to get a new, empty vector if the value is None.



impl Inventory {
    fn giveaway(&self, user_prefrence: Option<ShirtColor>) -> ShirtColor {
        user_prefrence.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // TEST DIFF CASES
    // closure_capturing_imm_ref();
    // closure_captuer_mut_ref();
    // take_ownership_explictly_in_closure();

    sort_test();
}

fn closure_capturing_imm_ref() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn closure_captuer_mut_ref() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // Uncomment below line to show an error of can't have an immutable ref
    // when there already is mutable ref in this scope;
    // println!("This won't work: {list:?}");

    borrows_mutably();
    println!("After calling closure: {list:?}");
}


use std::thread;

// doing this to move the value in new thread
fn take_ownership_explictly_in_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // We can't remove move, as need to move value in new thread
    // in case old thread completes first and it drops the value ref in 2nd thread
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

// Eg of FnMut bound trait closure, that sorts values
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sort_test() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}

fn fn_once_eg_with_error() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    list.sort_by_key(|r| {
        // Uncomment following to show error
        // sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");
}