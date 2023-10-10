use std::io::{self, Read};
/*
Short for: 
use std::io
use std::io::Read
*/ 


fn main() {
    for byte in io::stdin().bytes() {
        let character:char = byte.unwrap() as char;
        println!("{}", character);
        if character == 'q' { // NOTE: Characters in Rust are single quoted to work, not double quoted
            // this if statement ends the program if there is a character 'q' in the input
            break;
        }
    }
}
