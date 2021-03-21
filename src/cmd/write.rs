use super::{Command, CommandWrapper};
use crate::db;
use crate::error::PassError;

pub struct CommandWrite {}

impl Command for CommandWrite {
    type Args = ();
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandWrite {})
    }
    fn name(&self) -> &'static str {
        "write"
    }
    fn help(&self) -> &'static str {
        "Write the database"
    }
    fn run(&self, _: (), db: &mut db::Database) -> Result<(), PassError> {
        db.save(None);
        Ok(())
    }
    fn parse(
        &self,
        raw_args: &clap::ArgMatches,
        db: &mut db::Database,
    ) -> Result<(), PassError> {
        Ok(())
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .short_flag('W')
            .bin_name(Command::name(self))
            .about(Command::help(self))
            .setting(clap::AppSettings::DisableVersion)
    }
    fn repl_only(&self) -> bool {
        false
    }
}
