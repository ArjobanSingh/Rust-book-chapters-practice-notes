// fn mainEg1() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y and z are {x}, {y} and {z}");

    // arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // initialise array with five 3's
    let arr2 = [3; 5];
}