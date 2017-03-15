extern crate extern_crate;

// only in main.rs or lib.rs
mod module1;
mod module2;

use local_modules::LocalTrait
use local_modules::local_func

use dependency_modules::DepTrait
use dependency_modules::dep_func

use std_modules::StdTrait
use std_modules::std_func

// Main + Non-struct Functions

fn main_or_nonstruct() {

}


// Module Enums

enum SomeOptions {
    Win,
    Lose,
    Butts(u8)
}

// Module tuple structs

struct TupStruct(i32, i32, &str)

// Module C structs

struct ModStruct {
    field: String,
    other: [u8]
}

// Impls for methods and associated functions

impl ModStruct {
    // new(), constructors, or builders
    pub fn new() -> ModStruct {
        ...
    }

    // public methods and associated functions

    pub fn do_thing(&self) -> Option<i32> {
        Some(42)                                        // Implicit returns wherever possible
    }

    // private methods and associated functions

    fn work_in_secret() -> Result<u8, Error> {
        Ok(1023)
    }
}

// Impls for traits
// Trait implementations should follow the same order as above:
// Local, then external dependencies, then standard library

impl LocalTrait for ModStruct {

}

impl DepTrait for ModStruct {

}

impl Write for ModStruct {
    ...
}


// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
