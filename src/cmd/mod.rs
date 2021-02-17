pub mod add;
pub mod clear;
pub mod generate;
pub mod help;
pub mod info;
pub mod list;
pub mod meta;
pub mod quit;
pub mod remove;
pub mod update;
pub mod write;
pub mod yank;

use crate::db;

pub trait CommandWrapper {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn clap_app(&self) -> clap::App;
    fn parse_and_run(&self, raw_args: &clap::ArgMatches, db: &mut db::Database);
    fn repl_only(&self) -> bool;
}

impl<C: Command> CommandWrapper for C {
    fn name(&self) -> &'static str {
        self.name()
    }
    fn help(&self) -> &'static str {
        self.help()
    }
    fn parse_and_run(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) {
        self.run(self.parse(raw_args, db), db)
    }
    fn clap_app(&self) -> clap::App {
        self.clap_app()
    }
    fn repl_only(&self) -> bool {
        self.repl_only()
    }
}

pub trait Command {
    type Args;
    fn new() -> Box<dyn CommandWrapper>;
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn run(&self, options: Self::Args, db: &mut db::Database);
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> Self::Args;
    fn clap_app(&self) -> clap::App;
    fn repl_only(&self) -> bool;
}

pub type CommandVec = Vec::<Box<dyn CommandWrapper>>;

pub trait Commands {
    fn build() -> Self;
    fn find(&self, name: &str) -> Result<&Box<dyn CommandWrapper>, &'static str>;
}

impl Commands for CommandVec {
    fn build() -> CommandVec {
        vec![
            add::CommandAdd::new(),
            clear::CommandClear::new(),
            generate::CommandGenerate::new(),
            help::CommandHelp::new(),
            info::CommandInfo::new(),
            list::CommandList::new(),
            meta::CommandMeta::new(),
            quit::CommandQuit::new(),
            remove::CommandRemove::new(),
            update::CommandUpdate::new(),
            write::CommandWrite::new(),
            yank::CommandYank::new(),
        ]
    }

    fn find(&self, name: &str) -> Result<&Box<dyn CommandWrapper>, &'static str>
    {
        for command in self {
            if let Some(i) = command.name().find(name) {
                if i == 0 {
                    return Ok(command);
                }
            }
        }
        Err("Unknown command")
    }
}
