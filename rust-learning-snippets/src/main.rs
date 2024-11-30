fn main() {
    let yes = "y̆es";

    // We can even define variables using non-ascii chars. WoW
    let नमस्ते = "Damn didn't knew about this";
    println!("what {नमस्ते}");

    let char_indices: Vec<(usize, char)> = yes.char_indices().collect();
    let chars: Vec<char> = yes.chars().collect();

    // Char indices and chars actually gives
    println!("char_indices {:?}", char_indices);
    println!("chars vector {:?}", chars);

    // Diacritic is unicode scalar value
    println!("diacritic at 1 on whitespace char is: {}", yes.chars().nth(1).unwrap());

    // Following in not safe for ascii, as it gives garbage or byte value of multi-byte character
    println!("char as byte: {:?}", yes.as_bytes()[2] as char);

    // Uncommenting following will panic, trying to splice within multi-byte char
    // println!("slice error: {}", &yes[0..2])
}
