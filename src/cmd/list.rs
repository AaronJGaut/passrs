use super::{Command, CommandWrapper};
use crate::db;

pub struct CommandList {}

impl Command for CommandList {
    type Args = ();
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandList {})
    }
    fn name(&self) -> &'static str {
        "list"
    }
    fn help(&self) -> &'static str {
        "List all entries"
    }
    fn run(&self, _: (), db: &mut db::Database) {
        // TODO: use $PAGER or more for long output
        let records = db.get_records();
        println!("Listing {} accounts", records.len());
        for it in records.iter().enumerate() {
            println!("{:>5} {}", it.0, it.1.account);
        }
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> Result<(), String> {
        Ok(())
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('L')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
