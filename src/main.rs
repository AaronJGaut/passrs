use passrs::cmd::{self, Command, Commands};
use passrs::cli;
use passrs::db;

use clap::{App, crate_version};

fn main() {
    let commands = cmd::CommandVec::build();

    let mut app = App::new("passrs")
        .version(crate_version!())
        .about("CLI password manager");

    for command in &commands {
        if !command.repl_only() {
            app = app.subcommand(command.clap_app());
        }
    }
    let matches = app.get_matches();

    let mut db = db::Database::new("asdf");

    let mut ran_command = false;
    for command in &commands {
        if let Some(cmd_matches) = matches.subcommand_matches(command.name()) {
            command.parse_and_run(cmd_matches, &mut db);
            ran_command = true;
            break;
        }
    }
    if !ran_command {
        cli::repl(commands, db);
    }
}
