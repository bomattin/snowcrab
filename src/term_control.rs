use constants::{BOX_CHARS_BLD, BOX_CHARS_STD};

use termion::{async_stdin, color, style, clear, terminal_size, cursor, AsyncReader};
use termion::raw::{RawTerminal,IntoRawMode};
use termion::event::Key;
use termion::input::TermRead;

use std::io::{Read, Write, stdout, Stdout, stdin};
use std::thread::sleep;
use std::time::Duration;

// TODO: Offer options for Async In and Raw Out, with flags to indicate their state.

pub struct TermControl {
    stdin: AsyncReader,
    stdout: RawTerminal<Stdout>,
    read_buf: [u8;1024],
    write_buf: [u8;1024],
    key_buf: [u8;4],
    width: u16,
    height: u16
}

impl TermControl {
    pub fn new() -> TermControl {
        let (width, height) = terminal_size().unwrap();
        TermControl {
            stdin: async_stdin(),
            stdout: stdout().into_raw_mode().unwrap(),
            read_buf: [0; 1024],
            write_buf: [0; 1024],
            key_buf: [0;4],
            width: width,
            height: height
        }
    }

    fn border_draw(&mut self) {
        println!("{}", clear::All);
        write!(self.stdout, "{}{}", cursor::Goto(1,1), BOX_CHARS_BLD[2]);
        write!(self.stdout, "{}{}", cursor::Goto(self.width+1,1), BOX_CHARS_BLD[3]);
        write!(self.stdout, "{}{}", cursor::Goto(1,self.height+1), BOX_CHARS_BLD[4]);
        write!(self.stdout, "{}{}", cursor::Goto(self.width+1,self.height+1), BOX_CHARS_BLD[5]);

        for x in 2..self.width {
            write!(self.stdout, "{}{}", cursor::Goto(x,1), BOX_CHARS_BLD[0]);
            write!(self.stdout, "{}{}", cursor::Goto(x,self.height+1), BOX_CHARS_BLD[0]);
        }

        for y in 2..self.height {
            write!(self.stdout, "{}{}", cursor::Goto(1,y), BOX_CHARS_BLD[1]);
            write!(self.stdout, "{}{}", cursor::Goto(self.width+1,y), BOX_CHARS_BLD[1]);
        }
        self.stdout.flush().unwrap();
    }

    fn input_capture(&mut self) {
        let keycount = self.stdin.read(&mut self.key_buf);

        match keycount {
            Ok(n)   =>  writeln!(self.stdout, "{}{:?}", cursor::Goto(2,2), &self.key_buf).unwrap(),
            Err(e)  =>  writeln!(self.stdout, "{}Error : {:?}", cursor::Goto(10,10), e).unwrap()
        }

        self.key_buf = [0;4];
    }

    pub fn run(&mut self) {
        self.border_draw();
        loop {
            self.input_capture();
            sleep(Duration::from_millis(1000));
        }
    }
}
