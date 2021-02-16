use super::{Command, CommandWrapper};

pub struct ArgsInfo {
    show: bool,
    index: u32,
}

pub struct CommandInfo {
}

impl Command for CommandInfo {
    type Args = ArgsInfo;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandInfo {})
    }
    fn name(&self) -> &'static str { "info" }
    fn help(&self) -> &'static str { "Print info about an entry" }
    fn run(&self, opts: ArgsInfo) {
    }
    fn parse(&self, raw_args: &clap::ArgMatches) -> ArgsInfo {
        ArgsInfo { show: true, index: 0 }
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('I')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
