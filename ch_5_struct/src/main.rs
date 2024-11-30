struct User {
    id: u32,
    name: String,
    email: String,
    sign_in_count: u32
}

// Tuple struct
struct Color(u32, u32, u32);
struct Point(u32, u32, u32);

fn printColor(color: Color) {
    let Color (r, g, b) = color;
    println!("The color is RGB format is {r}{g}{b}")
}

fn main() {
    println!("Hello, world!");
    let user = User {
        id: 1,
        name: String::from("Arjoban"),
        email: String::from("some@email.com"),
        sign_in_count: 1
    };
    let black = Color(0, 0, 0);
    let start = Point(0, 0, 0);
    printColor(black);

    // This won't work obviously, same fields but different struct type
    // printColor(start);
    print!("{} with id as {} and email as {} have signed {} these many times", user.name, user.id, user.email, user.sign_in_count);
}
