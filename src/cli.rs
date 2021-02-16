use crate::cmd::{self, Command, Commands};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

use std::error::Error;
use std::io;


pub fn repl(commands: cmd::CommandVec) {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("\x1b[92;1mpassrs>\x1b[0m ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let words = to_args(&line);
                if words.len() > 0
                {
                    match commands.find(words[0]) {
                        Ok(cmd) => cmd.parse_and_run(&cmd.clap_app().get_matches()),
                        Err(err) => println!("Error: {}", err),
                    }
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                if confirm_interrupt() {
                    break
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}


pub fn confirm_interrupt() -> bool {
    println!("Press ctrl-c or ctrl-d again to quit or any key to continue");
    // need raw mode to read a single character
    match get_key() {
        Key::Ctrl('c') | Key::Ctrl('d') => true,
        _ => false,
    }
}

pub fn to_args(input: &str) -> Vec::<&str> {
    input.split_ascii_whitespace().collect()
}

pub fn get_key() -> Key {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    io::stdin().keys().next().unwrap().unwrap()
}

pub fn edit(prompt: &str) -> String
{
    String::from("")
}

