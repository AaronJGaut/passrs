use passrs::cli;
use passrs::cmd::{self, Commands};
use passrs::db;
use std::env;

use clap::{crate_version, App, Arg};

fn main() {
    let commands = cmd::CommandVec::build();

    let default_path = match env::var("PASSRS_DB") {
        Ok(val) => val,
        Err(_) => match env::var("PASSMAN_DB") {
            Ok(val) => val,
            Err(_) => match env::var("HOME") {
                Ok(val) => val + "/.passrs.json",
                Err(_) => String::from(".passrs.json"),
            },
        },
    };

    let mut app = App::new("passrs")
        .version(crate_version!())
        .about("CLI password manager")
        .arg(Arg::new("filepath")
            .about("Path of the accounts file")
            .short('f')
            .long("filepath")
            .default_value(default_path.as_str())
        );

    for command in &commands {
        if !command.repl_only() {
            app = app.subcommand(command.clap_app());
        }
    }
    let matches = app.get_matches();

    let mut db = db::Database::new(matches.value_of("filepath").unwrap());

    let mut ran_command = false;
    for command in &commands {
        if let Some(cmd_matches) = matches.subcommand_matches(command.name()) {
            if let Err(err) = command.parse_and_run(cmd_matches, &mut db) {
                println!("Error: {}", err);
            };
            if db.modified {
                db.save(None);
            }
            ran_command = true;
            break;
        }
    }
    if !ran_command {
        cli::repl(commands, db);
    }
}
