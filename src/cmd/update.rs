use super::{Command, CommandWrapper};

pub struct ArgsUpdate {
    account: bool,
    username: bool,
    password: bool,
    notes: bool,
    show: bool,
    index: u32,
}

pub struct CommandUpdate {
}

impl Command for CommandUpdate {
    type Args = ArgsUpdate;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandUpdate {})
    }
    fn name(&self) -> &'static str { "update" }
    fn help(&self) -> &'static str { "Update an entry" }
    fn run(&self, opts: ArgsUpdate) {
    }
    fn parse(&self, raw_args: &clap::ArgMatches) -> ArgsUpdate {
        ArgsUpdate {
            account: false,
            username: false,
            password: false,
            notes: false,
            show: false,
            index: 0
        }
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('U')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
