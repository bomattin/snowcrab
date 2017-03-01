extern crate termion;

mod constants;

use termion::{color, style, clear, terminal_size, cursor};
use termion::raw::{RawTerminal,IntoRawMode};
use termion::event::Key;
use std::io::{Write, Stdout, stdout};
use constants::{BOX_CHARS_BLD, BOX_CHARS_STD};

fn main() {

    init();
    println!("{}", clear::All);

}

fn init() {
    border_draw();
}

fn border_draw() {
    println!("{}", clear::All);

    let mut stdout = stdout().into_raw_mode().unwrap();
    let (width, height) = terminal_size().unwrap();

    // Corners
    // Stupid Goto is 1-indexed
    write!(stdout, "{}{}", cursor::Goto(1,1), BOX_CHARS_BLD[2]);
    write!(stdout, "{}{}", cursor::Goto(width+1,1), BOX_CHARS_BLD[3]);
    write!(stdout, "{}{}", cursor::Goto(1,height+1), BOX_CHARS_BLD[4]);
    write!(stdout, "{}{}", cursor::Goto(width+1,height+1), BOX_CHARS_BLD[5]);

    for x in 2..width {
        write!(stdout, "{}{}", cursor::Goto(x,1), BOX_CHARS_BLD[0]);
        write!(stdout, "{}{}", cursor::Goto(x,height+1), BOX_CHARS_BLD[0]);
    }

    for y in 2..height {
        write!(stdout, "{}{}", cursor::Goto(1,y), BOX_CHARS_BLD[1]);
        write!(stdout, "{}{}", cursor::Goto(width+1,y), BOX_CHARS_BLD[1]);
    }

}
