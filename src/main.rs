use rustyline::error::ReadlineError;
use rustyline::Editor;

use passrs;
use passrs::Command;
use clap::{Arg, App};

use std::error::Error;

fn repl() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("\n\x1b[92;1mpassrs>\x1b[0m ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let words = passrs::to_args(&line);
                if words.len() > 0
                {
                    match passrs::match_command(words[0]) {
                        Ok("add") => {
                            let cmd = passrs::CommandAdd::new();
                            cmd.parse_and_run();
                        }
                        Ok(cmd) => println!("Command: {}", cmd),
                        Err(err) => println!("Error: {}", err),
                    }
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                if passrs::confirm_interrupt() {
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

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("passrs")
        .version("0.1.0")
        .subcommand(App::new("add")
            .short_flag('A')
        )
        .subcommand(App::new("generate")
            .short_flag('G')
        )
        .subcommand(App::new("info")
            .short_flag('I')
        )
        .subcommand(App::new("list")
            .short_flag('L')
        )
        .subcommand(App::new("meta")
            .short_flag('M')
        )
        .subcommand(App::new("remove")
            .short_flag('R')
        )
        .subcommand(App::new("update")
            .short_flag('U')
        )
        .subcommand(App::new("write")
            .short_flag('W')
        )
        .subcommand(App::new("yank")
            .short_flag('Y')
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        passrs::cmd_add();
    } else {
        repl();
    }

    Ok(())
}
