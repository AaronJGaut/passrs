use crate::cmd::{self, Command, Commands};
use crate::db;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::error::Error;
use std::io::{self, Write};

use edit;

const EDITOR_TAIL: &'static str = "";

pub fn repl(commands: cmd::CommandVec, mut db: db::Database) {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("\x1b[92;1mpassrs>\x1b[0m ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let words = to_args(&line);
                if words.len() > 0 {
                    match commands.find(words[0]) {
                        Ok(cmd) => match &cmd.clap_app().try_get_matches_from(words) {
                            Ok(matches) => cmd.parse_and_run(matches, &mut db),
                            Err(err) => print!("{}", err),
                        },
                        Err(err) => println!("Error: {}", err),
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                if confirm_interrupt() {
                    break;
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
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

pub fn to_args(input: &str) -> Vec<&str> {
    input.split_ascii_whitespace().collect()
}

pub fn get_key() -> Key {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    io::stdin().keys().next().unwrap().unwrap()
}

pub fn read_editor(existing_text: &str, tail: &str) -> io::Result<String> {
    let lines = tail.lines();
    let mut pre_text = String::from(existing_text);
    for line in lines {
        pre_text += "\n# ";
        pre_text += line;
    }

    let post_text = edit::edit(pre_text)?;

    let lines = post_text.trim().lines();
    let mut notes = String::new();
    for line in lines {
        if !line.starts_with("#") {
            notes += line;
        }
    }
    Ok(notes)
}

pub fn clear() {
    print!("\x1b[2J\x1b[3J\x1b[1;1H");
}

pub fn read(prompt: &str, allow_empty: bool) -> Option<String> {
    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline(prompt) {
            Ok(text) => {
                if !allow_empty && text.as_str() == "" {
                    continue;
                } else {
                    return Some(text);
                }
            }
            Err(_) => return None,
        }
    }
}

pub fn read_hidden(prompt: &str, allow_empty: bool) -> Option<String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    loop {
        stdout.write_all(prompt.as_bytes()).unwrap();
        stdout.flush().unwrap();
        let pass = stdin.read_passwd(&mut stdout);
        stdout.write_all(b"\n");
        if let Ok(Some(pass)) = pass {
            if !allow_empty && pass == "" {
                continue;
            }
            return Some(pass);
        } else {
            return None;
        }
    }
}
