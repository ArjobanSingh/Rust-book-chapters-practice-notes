fn main() {
    println!("Hello, world!");
    let mut vec: Vec<i32> = Vec::new();
    vec.push(5);

    // create vector literal
    let v = vec![1, 2, 3, 4, 5];

    loop_test();
}

enum SpreadsheetCell {
    Integer(i32),
    Float(f64),
    Text(String),
}

fn multi_value_vec() {
    let row = vec![
        SpreadsheetCell::Integer(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

fn loop_test() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // mutable loop
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("Non updated value {i}");
        *i += 50;
        println!("Updated value {i}");
    }
}

fn out_of_bound_test() {
    let v = vec![1, 2, 3, 4, 5];
    // will throw error/panic
    let does_not_exist = &v[100];
    // will return Option, with None value
    let does_not_exist2 = v.get(100);
}

fn access_vec_elems() {
    let v = vec![1, 2, 3, 4, 5];

    // Get a ref to third element in vector
    let third_el: &i32 = &v[2];
    println!("The third element is {third_el}");

    // using get method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element using get is {third}"),
        None => println!("There is no third element."),
    }
}

fn mut_and_immut_error() {
    // can't have mutable and immutable thing ref at same time
    let mut v2 = vec![1, 2, 3, 4, 5];

    // THis won't be accessed after push, as in
    // same scope can't have mutable and immutable value
    // Though if would have copied the item, without borrowing with ref
    // It would have worked.
    let value = &v2[0];

    // Uncomment to check the error
    // v2.push(6);

    println!("The first element is: {}", value);
}

fn vec_item_ownership_error() {
    let vec = vec![String::from("Hello")];
    // As Vector keeps the ownership of its elements, strings
    // ownership can't be moved, and it also doesn't have copy trait.
    // Uncomment to check error
    // let str = vec[0];
}
