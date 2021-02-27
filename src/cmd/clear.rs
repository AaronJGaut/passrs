use super::{Command, CommandWrapper};
use crate::cli::clear;
use crate::db;

pub struct CommandClear {}

impl Command for CommandClear {
    type Args = ();
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandClear {})
    }
    fn name(&self) -> &'static str {
        "clear"
    }
    fn help(&self) -> &'static str {
        "Clear the terminal"
    }
    fn run(&self, _: (), db: &mut db::Database) {
        clear();
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> Result<(), String> {
        Ok(())
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new("clear")
            .about(Command::help(self))
            .short_flag('C')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
