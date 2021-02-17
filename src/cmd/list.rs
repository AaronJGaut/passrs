use super::{Command, CommandWrapper};
use crate::db;

pub struct CommandList {
}

impl Command for CommandList {
    type Args = ();
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandList {})
    }
    fn name(&self) -> &'static str { "list" }
    fn help(&self) -> &'static str { "List all entries" }
    fn run(&self, _: (), db: &mut db::Database) {
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> () {}
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('L')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
