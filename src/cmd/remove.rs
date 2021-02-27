use super::{Command, CommandWrapper};
use crate::{db, cli};

pub struct ArgsRemove {
    index: usize,
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
        "Remove an account"
    }
    fn run(&self, opts: ArgsRemove, db: &mut db::Database) {
    }
    fn parse(
        &self,
        raw_args: &clap::ArgMatches,
        db: &mut db::Database,
    ) -> Result<ArgsRemove, String> {
        let index = db::parse_index(db, raw_args.value_of("index").unwrap())?;
        Ok(ArgsRemove { index: index })
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .short_flag('R')
            .bin_name(Command::name(self))
            .about(Command::help(self))
            .setting(clap::AppSettings::DisableVersion)
            .arg(
                clap::Arg::new("index")
                    .about("Account index or name")
                    .takes_value(true)
                    .required(true),
            )
    }
    fn repl_only(&self) -> bool {
        false
    }
}
