use super::{Command, CommandWrapper};

pub struct ArgsMeta {
    show: bool,
}

pub struct CommandMeta {
}

impl Command for CommandMeta {
    type Args = ArgsMeta;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandMeta {})
    }
    fn name(&self) -> &'static str { "meta" }
    fn help(&self) -> &'static str { "Print information about the database" }
    fn run(&self, opts: ArgsMeta) {
    }
    fn parse(&self, raw_args: &clap::ArgMatches) -> ArgsMeta {
        ArgsMeta { show: true }
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .about(Command::help(self))
            .short_flag('M')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
