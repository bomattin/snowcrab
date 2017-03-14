extern crate termion;

mod constants;
mod term_control;

use term_control::TermControl;
use termion::clear;



fn main() {

    println!("{}", clear::All);
    let mut tc = TermControl::new();
    tc.run();
    println!("{}", clear::All);

}

// TODO: Let's build a command factory! https://en.wikipedia.org/wiki/Command_pattern#Java
