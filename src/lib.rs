use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

use std::io;

pub fn confirm_interrupt() -> bool {
    println!("Press ctrl-c or ctrl-d again to quit or any key to continue");
    // need raw mode to read a single character
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let b = io::stdin().keys().next().unwrap().unwrap();
    match b {
        Key::Ctrl('c') => true,
        Key::Ctrl('d') => true,
        _ => false,
    }
}
