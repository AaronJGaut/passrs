use super::{Command, CommandWrapper};

pub struct ArgsGenerate {
    length: u32,
    alphanumeric: bool,
    show: bool,
}

pub struct CommandGenerate {
}

impl Command for CommandGenerate {
    type Args = ArgsGenerate;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandGenerate {})
    }
    fn name(&self) -> &'static str { "generate" }
    fn help(&self) -> &'static str { "Generate a password" }
    fn run(&self, opts: ArgsGenerate) {
    }
    fn parse(&self, raw_args: &clap::ArgMatches) -> ArgsGenerate {
        ArgsGenerate { show: true, length: 20, alphanumeric: false }
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('G')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
