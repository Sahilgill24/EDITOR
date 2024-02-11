use core::panic;
use std::io::{self, stdout};
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;



pub struct Editor { }

fn die(e: std::io::Error){
    panic!("{}",e);
}



impl Editor{
    pub fn run(&self) {

        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c)=>{
                        if c.is_control() {
                            println!("{:?} \r",c as u8);
                        }
                        else {
                            println!("{:?} ({})",c as u8,c);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}\r",key)
                },
                Err(err) => die(err),
            }
        }
    }
    pub fn default() -> Self {
        Editor {  }
    }
}