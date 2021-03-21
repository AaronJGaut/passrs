use super::{Command, CommandWrapper};
use crate::db;
use crate::error::PassError;

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
    fn run(&self, opts: ArgsMeta, db: &mut db::Database) -> Result<(), PassError> {
        db.require_loaded()?;
        println!("filepath: {}", db.filepath);
        println!("modified: {}", db.modified);
        if opts.show {
            let password = &db.data.as_ref().unwrap().password;
            let password = std::str::from_utf8(password.as_slice()).unwrap();
            println!("master password: {}", password);
        }
        Ok(())
    }
    fn parse(
        &self,
        raw_args: &clap::ArgMatches,
        db: &mut db::Database,
    ) -> Result<ArgsMeta, PassError> {
        Ok(ArgsMeta { show: raw_args.is_present("show") })
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .short_flag('M')
            .bin_name(Command::name(self))
            .about(Command::help(self))
            .setting(clap::AppSettings::DisableVersion)
            .arg(
                clap::Arg::new("show")
                    .about("Show the master password")
                    .short('s')
                    .long("show")
            )
    }
    fn repl_only(&self) -> bool {
        false
    }
}
