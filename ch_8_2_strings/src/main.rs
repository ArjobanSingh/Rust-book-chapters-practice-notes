fn main() {
    println!("Hello, world!");
    str_concatenation();
    str_concatenation_2();
    concatenate_multiple();
    format_str();
    ref_str_idx();
}

fn str_concatenation() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let s2 = "baz".to_string();
    // cannot pass string, need to pass string slice
    // s.push_str(s2);
    s.push_str(&s2);

    println!("The concatenated string is: {s}");
}

fn str_concatenation_2() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // we can only add ref to second string
    // Though s2 is String and &s2 is &String, compiler coerce it to &str
    // using &s2[..]
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("Concated string: {s3}");
}

fn concatenate_multiple() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // less readable
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Manual concatenation: {s}");
}

fn format_str() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("Format str: {s}");
}

fn ref_str_idx() {
    let hello = "Здравствуйте";

    // NOTE: following will not compile
    // let answer = &hello[0];

    // This gets the first 4 bytes of the string
    // and then print that, not first 4 chars
    let s = &hello[0..4];
    println!("What is this? {s}");

    print!("Char --> ");
    for c in hello.chars() {
        print!("{c}")
    }
}
