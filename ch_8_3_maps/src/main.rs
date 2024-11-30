use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert("Yellow".to_string(), 50);


    // get value from map
    let team_name = String::from("Blue");

    // The get method returns an Option<&V>; if there’s no value for that key in the hash map,
    // get will return None. This program handles the Option by calling copied to get an Option<i32>
    // rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn’t have an entry for the key.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }


    // Either return ref to value for yellow or insert it with 50
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}
