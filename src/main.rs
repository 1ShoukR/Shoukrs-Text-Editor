use std::io::{self, Read, stdout};
use crossterm::terminal::enable_raw_mode;
/*
Short for: 
use std::io
use std::io::Read
*/ 


fn main() {
    let _raw_mode = enable_raw_mode().unwrap();

    for byte in io::stdin().bytes() {
        let byte = byte.unwrap();
        let character = byte as char;
        if character.is_control() {
            println!("{:?} \r", byte);
        } else {
            println!("{:?} ({})\r", byte, character);
        }




        // let b = b.unwrap();
        // let character = b as char;
        // if character == 'q' { // NOTE: Characters in Rust are single quoted to work, not double quoted
        //     // this if statement ends the program if there is a character 'q' in the input
        //     break;
        // }
    }
}
