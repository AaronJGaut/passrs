use super::{Command, CommandWrapper};

pub struct CommandClear {
}

impl Command for CommandClear {
    type Args = ();
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandClear {})
    }
    fn name(&self) -> &'static str { "clear" }
    fn help(&self) -> &'static str { "Clear the terminal" }
    fn run(&self, _: ()) {
        print!("\x1b[2J\x1b[3J\x1b[1;1H");
    }
    fn parse(&self, raw_args: &clap::ArgMatches) -> () {}
    fn clap_app(&self) -> clap::App {
        clap::App::new("clear")
            .about(Command::help(self))
            .short_flag('C')
    }
    fn repl_only(&self) -> bool {
        false
    }
}
