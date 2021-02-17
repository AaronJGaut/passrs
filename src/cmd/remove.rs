use super::{Command, CommandWrapper};
use crate::db;

pub struct ArgsRemove {
    index: u32,
}

pub struct CommandRemove {}

impl Command for CommandRemove {
    type Args = ArgsRemove;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandRemove {})
    }
    fn name(&self) -> &'static str {
        "remove"
    }
    fn help(&self) -> &'static str {
        "Remove an entry"
    }
    fn run(&self, opts: ArgsRemove, db: &mut db::Database) {}
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> ArgsRemove {
        ArgsRemove { index: 0 }
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('R')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
