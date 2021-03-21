use super::{Command, CommandWrapper};
use crate::{db, cli};
use std::process::exit;
use rustyline::error::ReadlineError;
use crate::error::PassError;

pub struct ArgsQuit {
    force: bool,
    write: bool,
}

pub struct CommandQuit {}

impl Command for CommandQuit {
    type Args = ArgsQuit;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandQuit {})
    }
    fn name(&self) -> &'static str {
        "quit"
    }
    fn help(&self) -> &'static str {
        "Quit interactive passrs"
    }
    fn run(&self, opts: ArgsQuit, db: &mut db::Database) -> Result<(), PassError> {
        if !db.modified {
            exit(0);
        }
        if opts.force {
            exit(0);
        }
        if opts.write {
            db.save(None);
            exit(0);
        }
        if cli::read_confirm("Write unsaved changes?", Some(true))? {
            db.save(None);
            exit(0);
        } else if cli::read_confirm("Quit without saving?", Some(true))? {
            exit(0);
        }
        Ok(())
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> Result<ArgsQuit, PassError> {
        Ok(ArgsQuit {
            force: raw_args.is_present("force"),
            write: raw_args.is_present("write"),
        })
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .short_flag('Q')
            .bin_name(Command::name(self))
            .about(Command::help(self))
            .setting(clap::AppSettings::DisableVersion)
            .arg(
                clap::Arg::new("force")
                    .conflicts_with("write")
                    .about("Quit without saving")
                    .short('f')
                    .long("force"),
            )
            .arg(
                clap::Arg::new("write")
                    .about("Write unsaved changes")
                    .short('w')
                    .long("write"),
            )
    }
    fn repl_only(&self) -> bool {
        true
    }
}
