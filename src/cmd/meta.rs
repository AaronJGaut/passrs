use super::{Command, CommandWrapper};
use crate::db;

pub struct ArgsMeta {
    show: bool,
}

pub struct CommandMeta {}

impl Command for CommandMeta {
    type Args = ArgsMeta;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandMeta {})
    }
    fn name(&self) -> &'static str {
        "meta"
    }
    fn help(&self) -> &'static str {
        "Display database metadata"
    }
    fn run(&self, opts: ArgsMeta, db: &mut db::Database) {}
    fn parse(
        &self,
        raw_args: &clap::ArgMatches,
        db: &mut db::Database,
    ) -> Result<ArgsMeta, String> {
        Ok(ArgsMeta { show: true })
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('M')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
