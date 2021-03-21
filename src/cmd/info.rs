use super::{Command, CommandWrapper};
use crate::db;
use crate::error::PassError;

pub struct ArgsInfo {
    show: bool,
    index: usize,
}

pub struct CommandInfo {}

impl Command for CommandInfo {
    type Args = ArgsInfo;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandInfo {})
    }
    fn name(&self) -> &'static str {
        "info"
    }
    fn help(&self) -> &'static str {
        "Display info for an account"
    }
    fn run(&self, opts: ArgsInfo, db: &mut db::Database) -> Result<(), PassError> {
        let record = &db.get_records()?[opts.index];
        println!("Account: {}", record.account);
        if let Some(username) = record.username.as_ref() {
            println!("Username: {}", username);
        }
        if opts.show {
            println!("Password: {}", record.password);
        }
        if let Some(notes) = record.notes.as_ref() {
            println!("Notes:");
            for line in notes.lines() {
                println!("\t{}", line);
            }
        }
        Ok(())
    }
    fn parse(
        &self,
        raw_args: &clap::ArgMatches,
        db: &mut db::Database,
    ) -> Result<ArgsInfo, PassError> {
        let index = db::parse_index(db, raw_args.value_of("index").unwrap())?;
        Ok(ArgsInfo {
            show: raw_args.is_present("show"),
            index: index,
        })
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .short_flag('I')
            .bin_name(Command::name(self))
            .about(Command::help(self))
            .setting(clap::AppSettings::DisableVersion)
            .arg(
                clap::Arg::new("index")
                    .about("Account index or name")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                clap::Arg::new("show")
                    .about("Print the password")
                    .short('s')
                    .long("show"),
            )
    }
    fn repl_only(&self) -> bool {
        false
    }
}
