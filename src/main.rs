use std::io::{Result};
use crossterm::{terminal, event::{self, KeyCode, Event::*}};
/*
Short for: 
use std::io
use std::io::Read
*/ 


fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        if let Ok(event ) = event::read() {
            if let Key(key_event) = event {
                if key_event.code == KeyCode::Char('q') {
                    break;
                } else {
                    println!("{:?}\n", event);
                }
            }
        } else {
            break
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())




        // let b = b.unwrap();
        // let character = b as char;
        // if character == 'q' { // NOTE: Characters in Rust are single quoted to work, not double quoted
        //     // this if statement ends the program if there is a character 'q' in the input
        //     break;
        // }
    }
