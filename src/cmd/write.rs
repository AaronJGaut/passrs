use super::{Command, CommandWrapper};
use crate::db;

pub struct ArgsWrite {
    //encryption: DbEncryption,
    //format: DbFormat,
    password: bool,
    show: bool,
    filepath: String,
}

pub struct CommandWrite {}

impl Command for CommandWrite {
    type Args = ArgsWrite;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandWrite {})
    }
    fn name(&self) -> &'static str {
        "write"
    }
    fn help(&self) -> &'static str {
        "Write the database"
    }
    fn run(&self, opts: ArgsWrite, db: &mut db::Database) {}
    fn parse(
        &self,
        raw_args: &clap::ArgMatches,
        db: &mut db::Database,
    ) -> Result<ArgsWrite, String> {
        Ok(ArgsWrite {
            password: false,
            show: true,
            filepath: "".to_string(),
        })
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('W')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
