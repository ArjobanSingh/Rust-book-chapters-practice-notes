enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Option enum
    let some_number = Option::Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // we need to use match to use values of Option::enum
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // Using match pattern for coints

    let val = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Value of Dime is {val}");

    if_let_eg();
}

fn if_let_eg() {
    let config_max = Some(3u8);
    // If we only want to cover only one variant and rest to default
    // currently we've to use boilerplate empty arm.
    match config_max {
        Some(max) => println!("The maximun value is: {max}"),
        _ => (),
    }

    // We can fix this using if let
    // It's like defined a Some(variable) using some variable
    // But actually expression(config_max) is given to match and pattern is first arm
    if let Option::Some(max) = config_max {
        println!("Printing from shorter syntax! {max}");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // We can omit Option::, just for reference
        Option::None => Option::None,
        Some(i) => Some(i + 1),
    }
}

// Example of using catch-all with unused variable and no function in match
// let dice_roll = 9;
// match dice_roll {
//     3 => add_fancy_hat(),
//     7 => remove_fancy_hat(),
//     _ => (),
// }

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}

