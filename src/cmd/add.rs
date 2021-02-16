use super::{Command, CommandWrapper};

pub struct ArgsAdd {
    show: bool,
}

pub struct CommandAdd {
}

impl Command for CommandAdd {
    type Args = ArgsAdd;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandAdd {})
    }
    fn name(&self) -> &'static str { "add" }
    fn help(&self) -> &'static str { "Add a new entry" }
    fn run(&self, opts: ArgsAdd) {
        if opts.show {
            println!("adding");
        }
    }
    fn parse(&self, raw_args: &clap::ArgMatches) -> ArgsAdd {
        ArgsAdd { show: true }
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('A')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
