// Check this mod to understand Traits
mod aggregator;

// Importing Tweet and Summary Trait, to call summarise method on Tweet type
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    aggregator::notify(&tweet);
}
