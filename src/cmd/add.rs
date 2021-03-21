use super::{Command, CommandWrapper};
use crate::cli;
use crate::db::{self, Record};
use rustyline::error::ReadlineError;
use crate::error::PassError;

pub struct ArgsAdd {
    show: bool,
    account: String,
}

pub struct CommandAdd {}

impl Command for CommandAdd {
    type Args = ArgsAdd;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandAdd {})
    }
    fn name(&self) -> &'static str {
        "add"
    }
    fn help(&self) -> &'static str {
        "Add a new account"
    }
    fn run(&self, opts: ArgsAdd, db: &mut db::Database) -> Result<(), PassError> {
        // Calling explicitly so loading/creation occurs before any add prompts
        db.require_loaded()?;

        let user = cli::read("Username: ", true)?;
        let user = if user.is_empty() { None } else { Some(user) };

        let pass = if opts.show {
            cli::read("Password: ", false)
        } else {
            cli::create_password(
                "Password: ",
                "Repeat to confirm: ",
                "Mismatch. Please try again.",
            )
        }?;

        let notes = cli::read_editor("", "Enter any notes above")?;
        let notes = if notes.is_empty() { None } else { Some(notes) };

        db.add_record(Record {
            account: opts.account,
            username: user,
            password: pass,
            notes: notes,
        })?;

        Ok(())
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> Result<ArgsAdd, PassError> {
        Ok(ArgsAdd {
            show: raw_args.is_present("show"),
            account: String::from(raw_args.value_of("account").unwrap()),
        })
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .short_flag('A')
            .bin_name(Command::name(self))
            .about(Command::help(self))
            .setting(clap::AppSettings::DisableVersion)
            .arg(
                clap::Arg::new("account")
                    .about("Name of the account")
                    .takes_value(true)
                    .required(true)
            )
            .arg(
                clap::Arg::new("show")
                    .about("Show the password during entry")
                    .short('s')
                    .long("show")
            )
    }
    fn repl_only(&self) -> bool {
        false
    }
}
