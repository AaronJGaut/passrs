use super::{Command, CommandWrapper};
use crate::db;
use std::process::exit;

pub struct CommandQuit {}

impl Command for CommandQuit {
    type Args = ();
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandQuit {})
    }
    fn name(&self) -> &'static str {
        "quit"
    }
    fn help(&self) -> &'static str {
        "Quit interactive passrs"
    }
    fn run(&self, _: (), db: &mut db::Database) {
        exit(0);
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> Result<(), String> {
        Ok(())
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('Q')
    }
    fn repl_only(&self) -> bool {
        true
    }
}
