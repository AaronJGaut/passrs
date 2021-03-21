use crate::cmd::{self, Command, Commands};
use crate::{db, error};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::error::Error;
use std::io::{self, Write};

use edit;

pub fn repl(commands: cmd::CommandVec, mut db: db::Database) {
    println!("=== passrs interactive mode ===");
    println!("Enter help for a list of commands or quit to quit.");
    let mut rl = Editor::<()>::new();
    loop {
        let status_str = if db.modified {
            "*"
        } else if db.loaded() {
            ""
        } else {
            "🔒"
        };
        let prompt = format!("\x1b[92;1mpassrs{}>\x1b[0m ", status_str);
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let words = to_args(&line);
                if words.len() > 0 {
                    match commands.find(words[0]) {
                        Ok(cmd) => match &cmd.clap_app().try_get_matches_from(words) {
                            Ok(matches) => match cmd.parse_and_run(matches, &mut db) {
                                Ok(_) | Err(error::PassError::Interrupt) => continue,
                                Err(err) => println!("Error: {}", err),
                            },
                            Err(err) => print!("{}", err),
                        },
                        Err(_) => println!(
                            "Unknown command. Enter help for a list of commands or quit to quit."
                        ),
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
    println!("Press ctrl-c or ctrl-d again to quit without saving or any key to continue.");
    match get_key() {
        Key::Ctrl('c') | Key::Ctrl('d') => true,
        _ => false,
    }
}

pub fn to_args(input: &str) -> Vec<&str> {
    input.split_ascii_whitespace().collect()
}

pub fn get_key() -> Key {
    // need raw mode to read a single character
    let _stdout = io::stdout().into_raw_mode().unwrap();
    io::stdin().keys().next().unwrap().unwrap()
}

pub fn read_editor(existing_text: &str, tail: &str) -> Result<String, error::PassError> {
    let lines = tail.lines();
    let mut pre_text = String::from(existing_text);
    for line in lines {
        pre_text += "\n# ";
        pre_text += line;
    }

    let post_text = edit::edit(pre_text)?;

    let lines = post_text.trim().lines();
    let notes = lines.filter(|x| !x.starts_with("#")).collect::<Vec<&str>>().join("\n");
    Ok(notes)
}

pub fn clear() {
    print!("\x1b[2J\x1b[3J\x1b[1;1H");
}

pub fn read(prompt: &str, allow_empty: bool) -> Result<String, error::PassError> {
    let mut rl = Editor::<()>::new();
    loop {
        let input = rl.readline(prompt)?;
        if !allow_empty && input.is_empty() {
            continue;
        }
        return Ok(input);
    }
}

pub fn read_hidden(prompt: &str, allow_empty: bool) -> Result<String, error::PassError> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    loop {
        stdout.write_all(prompt.as_bytes()).unwrap();
        stdout.flush().unwrap();
        let pass = stdin.read_passwd(&mut stdout)?;
        stdout.write_all(b"\n").unwrap();
        stdout.flush().unwrap();
        match pass {
            Some(pass) => {
                if !allow_empty && pass == "" {
                    continue;
                } else {
                    return Ok(pass);
                }
            }
            None => return Err(error::PassError::Interrupt),
        }
    }
}

pub fn read_confirm(prompt: &str, default: Option<bool>) -> Result<bool, error::PassError> {
    let prompt = String::from(prompt) + match default {
        Some(default) => if default {
            " [Y/n]: "
        } else {
            " [y/N]: "
        },
        None => " [y/n]: ",
    };
    loop {
        let raw_input = match default {
            Some(default) => read(prompt.as_str(), true)?,
            None => read(prompt.as_str(), false)?,
        };
        let input = raw_input.clone().to_ascii_lowercase();
        if input == "y" || input == "yes" {
            return Ok(true)
        } else if input == "n" || input == "no" {
            return Ok(false)
        } else if input.is_empty() {
            return Ok(default.unwrap());
        }
        println!("Invalid input: \"{}\"", raw_input);
    }
}

pub fn create_password(
    prompt: &str,
    confirm: &str,
    mismatch: &str,
) -> Result<String, error::PassError> {
    loop {
        let pass1 = read_hidden(prompt, false)?;
        let pass2 = read_hidden(confirm, false)?;
        if pass1 == pass2 {
            return Ok(String::from(pass1));
        } else {
            println!("{}", mismatch);
        }
    }
}
