use super::{Command, CommandWrapper};
use crate::{cli, db};
use crate::error::PassError;

pub struct ArgsUpdate {
    account: bool,
    username: bool,
    password: bool,
    show: bool,
    notes: bool,
    index: usize,
}

pub struct CommandUpdate {}

impl Command for CommandUpdate {
    type Args = ArgsUpdate;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandUpdate {})
    }
    fn name(&self) -> &'static str {
        "update"
    }
    fn help(&self) -> &'static str {
        "Update one or more account fields"
    }
    fn run(&self, opts: ArgsUpdate, db: &mut db::Database) -> Result<(), PassError> {
        let mut record = db.get_records()?[opts.index].clone();
        if opts.account {
            record.account = cli::read("New account name: ", false)?;
        }
        if opts.username {
            let user = cli::read("New username: ", true)?;
            record.username = if user.is_empty() { None } else { Some(user) };
        }
        if opts.password {
            record.password = if opts.show {
                cli::read("New password: ", false)
            } else {
                cli::create_password(
                    "New password: ",
                    "Repeat to confirm: ",
                    "Mismatch. Please try again.",
                )
            }?;
        }
        if opts.notes {
            let old_notes = match &record.notes {
                Some(notes) => &notes,
                None => "",
            };
            let notes = cli::read_editor(old_notes, "Enter any notes above")?;
            record.notes = if notes.is_empty() { None } else { Some(notes) };
        }
        db.update_record(opts.index, record)?;
        Ok(())
    }
    fn parse(
        &self,
        raw_args: &clap::ArgMatches,
        db: &mut db::Database,
    ) -> Result<ArgsUpdate, PassError> {
        let index = raw_args.value_of("index").unwrap();
        let index = db::parse_index(db, index)?;
        Ok(ArgsUpdate {
            account: raw_args.is_present("account"),
            username: raw_args.is_present("username"),
            password: raw_args.is_present("password"),
            show: raw_args.is_present("show"),
            notes: raw_args.is_present("notes"),
            index: index,
        })
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .short_flag('U')
            .bin_name(Command::name(self))
            .about(Command::help(self))
            .setting(clap::AppSettings::DisableVersion)
            .arg(
                clap::Arg::new("account")
                    .about("Update account name")
                    .short('a')
                    .long("account")
            )
            .arg(
                clap::Arg::new("username")
                    .about("Update username")
                    .short('u')
                    .long("username")
            )
            .arg(
                clap::Arg::new("password")
                    .about("Update password")
                    .short('p')
                    .long("password")
            )
            .arg(
                clap::Arg::new("show")
                    .about("Show password when updating it")
                    .short('s')
                    .long("show")
            )
            .arg(
                clap::Arg::new("notes")
                    .about("Update notes")
                    .short('n')
                    .long("notes")
            )
            .arg(
                clap::Arg::new("index")
                    .about("Account index or name")
                    .takes_value(true)
                    .required(true)
            )
    }
    fn repl_only(&self) -> bool {
        false
    }
}
