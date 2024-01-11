use std::{io::{Result, self}, time::Duration};
use crossterm::{
    terminal, event::{self, KeyCode,  KeyEvent, Event::*, poll, KeyModifiers}
};

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    loop {
        let mut c = None;

        match poll(Duration::from_millis(100)) {
            Ok(true) => {
                if let Ok(Key(key_event)) = event::read() {
                    c = Some(key_event);
                } else {
                    die("Read Failed");
                }
            }
            Ok(false) => {}
            Err(_) => {
                die("Poll Failed");
            }
        }

        if let Some(c) = c {
            if c.code == KeyCode::Char('c') && c.modifiers.contains(KeyModifiers::CONTROL) {
                break;
            } else {
                println!("{:?}\r", c)
            }
        } else {
            println!("no key\r")
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

fn die<S: Into<String>>(message: S) {
    let _ = terminal::disable_raw_mode();
    eprintln!("{}: {}", message.into(), std::io::Error::last_os_error());
    std::process::exit(1)
}