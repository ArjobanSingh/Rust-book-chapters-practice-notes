// fn mainBorrowingEg() {
//     let mut s = String::from("hello");  // s comes into scope
//     // change(&mut s);
//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here
//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn change(str: &mut String) {
//     str.push_str(", world!");
// }

// fn mainPrevent2MutRefw() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2); 
// }

fn main() {
    let ref_to_nothing = dangle();
    println!("{ref_to_nothing}");

    let str1 = String::from("");
    // let str2 = str1;
    // println!("What str1 {str1}");
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}
// NOTE: this will not work, as ref to the string s will not work
// fn dangle() -> &str {
//     let s = String::from("hello");
//     &s[..]
// }
