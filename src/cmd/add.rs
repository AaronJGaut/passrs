use super::{Command, CommandWrapper};
use crate::cli;
use crate::db::{self, Record};

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
        "Add a new entry"
    }
    fn run(&self, opts: ArgsAdd, db: &mut db::Database) {
        db.require_loaded();
        if opts.show {
            println!("adding");
        }
        let username = cli::read("Username: ", true).unwrap();
        let username = if username.is_empty() {
            None
        } else {
            Some(username)
        };
        let password = if opts.show {
            cli::read("Password: ", false).unwrap()
        } else {
            cli::create_password(
                "Password: ",
                "Repeat to confirm: ",
                "Mismatch. Please try again.",
            )
            .unwrap()
        };
        let notes = cli::read_editor("", "Enter any notes above").unwrap();
        let notes = if notes.is_empty() { None } else { Some(notes) };
        db.add_record(Record {
            account: opts.account,
            username: username,
            password: password,
            notes: Some(notes.unwrap()),
        });
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> ArgsAdd {
        ArgsAdd {
            show: raw_args.is_present("show"),
            account: String::from(raw_args.value_of("account").unwrap()),
        }
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
                    .required(true),
            )
            .arg(
                clap::Arg::new("show")
                    .about("Show the password during entry")
                    .short('s')
                    .long("show"),
            )
    }
    fn repl_only(&self) -> bool {
        false
    }
}
