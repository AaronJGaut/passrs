use crate::cmd::{self, Command, CommandWrapper, Commands};
use crate::db;

pub struct CommandHelp {}

impl Command for CommandHelp {
    type Args = ();
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandHelp {})
    }
    fn name(&self) -> &'static str {
        "help"
    }
    fn help(&self) -> &'static str {
        "Print this message"
    }
    fn run(&self, _: (), db: &mut db::Database) {
        // TODO: use the repl's CommandVec
        let commands = cmd::CommandVec::build();
        println!("Commands");
        for command in commands {
            println!("{:>12}: {}", command.name(), command.help());
        }
        println!("Enter <COMMAND> --help for usage details");
        println!("Commands can be abbreviated (eg: u for update)");
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> Result<(), String> {
        Ok(())
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('H')
    }
    fn repl_only(&self) -> bool {
        true
    }
}
