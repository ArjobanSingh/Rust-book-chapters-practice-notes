fn main() {
    let s = String::from("hello world");
    let refer = &s;
    let word = first_word(refer);
    let word2 = word;
    // s.clear(); // this will throw error, as immutable reference to s is being used in println
    
    // can safely access word, even after reassign to word2, as word is reference
    println!("The first word is: {word}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    check_copy();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // let ans = String::from("");
    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..idx];
        }
    }

    &s[..]
}

fn check_copy() {
    let prim_tup = (1, 2);
    let prim_tup2 = prim_tup;

    // can be accessed, because a tuple of primitive types are copied
    let (one, two) = prim_tup;

}