// traits are kinda like interfaces from other langs
// Like defining the method signatrues, which other
// Types will implement, which wants to implement this trait

// One restriction to note is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate.

use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default fn implementation, that ca be overriden
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// it can get some type which implements the Summary Trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Basically above is Syntatic sugar for following
// Basically Trait bounding the generic
pub fn notify_long<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_impl_multi_trait(item: &(impl Summary + Display)) {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement Summary Trait(Interface) for NewsArticle Type
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// The fn format becomes less readable when multiple generic bound multiple traits
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // ... actual logic
    1
}

// Use the following readable way
fn some_function_readable<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ... actual logic
    1
}

// Return type that implements Trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Som points to consider: Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


// Conditionally implement Trait for Type if that implements other Trait
// Blanked Implementation:
// impl<T: Display> ToString for T {
//     // --snip--
// }