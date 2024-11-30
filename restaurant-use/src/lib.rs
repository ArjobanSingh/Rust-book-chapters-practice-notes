use std::fmt::Result;
use std::io::Result as IoResult; // Alias eg
// Use glob operator to bring everything in scope
use std::collections::*;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

mod front_of_house;

// bring long things into scope to use in shorthand way
// This is only valid in this scope, and not in child scope
pub use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // go to scope where parent is defined and get hosting
        super::hosting::add_to_waitlist();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
