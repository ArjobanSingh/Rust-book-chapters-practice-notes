fn main() {
    let value = print_labeled_measurement(5, 'h');
    println!("The returned value is {value}");
}

fn print_labeled_measurement(value: i32, unit_label: char) -> i32 {
    println!("The measurement is: {value}{unit_label}");
    5
}