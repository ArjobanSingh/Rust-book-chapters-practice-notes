fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // dangling_ref_fn();
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.


// fn with generic lifetime parameters
// Basically saying result value is valid as long as both the params are valid
// 'a specifying that all refs in this fntion must have same lifetime 'a.

// In practice, it means that the lifetime of the reference returned by the
// longest function is the same as the smaller of the lifetimes of the
// values referred to by the function arguments. These relationships are what
// we want Rust to use when analyzing this code.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Rust trying to prevent this bug, by annotating lifetimes
// fn dangling_ref_fn() {
//     let s1 = String::from("long string");
//     let result;
    
//     {
//         let s2 = String::from("short");
//         result = longest(s1.as_str(), s2.as_str());
//     }  // s2 goes out of scope here
    
//     println!("The longest string is: {}", result);
// }

// fn dangling_ref() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {r}");
// }


// Lifetime Refs in structs

// This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt <'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce<'b>(&'b self, announcement: &'b str) -> &'b str {
        println!("Announcing now");
        if false { return announcement }
        self.part
    }
}

fn struct_usage() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}


// Example of Generic Type Params, Trait bounds and lifetimes together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}