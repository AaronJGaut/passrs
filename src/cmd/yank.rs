extern crate copypasta;

use super::{Command, CommandWrapper};
use crate::{cli, db};
use copypasta::{ClipboardContext, ClipboardProvider};
use crate::error::PassError;

pub struct ArgsYank {
    index: usize,
}

pub struct CommandYank {}

impl Command for CommandYank {
    type Args = ArgsYank;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandYank {})
    }
    fn name(&self) -> &'static str {
        "yank"
    }
    fn help(&self) -> &'static str {
        "Copy the username/password to clipboard"
    }
    fn run(&self, opts: ArgsYank, db: &mut db::Database) -> Result<(), PassError> {
        let record = &db.get_records()?[opts.index];
        let mut ctx = ClipboardContext::new().unwrap();
        let old_content = ctx.get_contents();
        if let Some(username) = record.username.as_ref() {
            ctx.set_contents(username.clone());
            cli::read("Username yanked. Press enter to continue.", true);
        }
        ctx.set_contents(record.password.clone());
        cli::read("Password yanked. Press enter to continue.", true);
        match old_content {
            Ok(content) => {
                let result = ctx.set_contents(content);
                println!("Clipboard content restored.");
            }
            Err(_) => {
                ctx.set_contents(String::from(""));
            }
        }
        Ok(())
    }
    fn parse(
        &self,
        raw_args: &clap::ArgMatches,
        db: &mut db::Database,
    ) -> Result<ArgsYank, PassError> {
        let index = db::parse_index(db, raw_args.value_of("index").unwrap())?;
        Ok(ArgsYank { index: index })
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .short_flag('Y')
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
