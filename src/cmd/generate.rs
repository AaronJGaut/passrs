use super::{Command, CommandWrapper};
use crate::db;

pub struct ArgsGenerate {
    length: u32,
    alphanumeric: bool,
    show: bool,
}

pub struct CommandGenerate {}

impl Command for CommandGenerate {
    type Args = ArgsGenerate;
    fn new() -> Box<dyn CommandWrapper> {
        Box::new(CommandGenerate {})
    }
    fn name(&self) -> &'static str {
        "generate"
    }
    fn help(&self) -> &'static str {
        "Generate a secret and copy it to clipboard"
    }
    fn run(&self, opts: ArgsGenerate, db: &mut db::Database) {
        println!("{} {} {}", opts.show, opts.length, opts.alphanumeric);
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> ArgsGenerate {
        ArgsGenerate {
            show: raw_args.is_present("show"),
            length: u32::from_str_radix(raw_args.value_of("length").unwrap(), 10).unwrap(),
            alphanumeric: raw_args.is_present("alphanumeric"),
        }
    }
    fn clap_app(&self) -> clap::App {
        clap::App::new(Command::name(self))
            .short_flag('G')
            .bin_name(Command::name(self))
            .about(Command::help(self))
            .setting(clap::AppSettings::DisableVersion)
            .arg(
                clap::Arg::new("length")
                    .about("Num. characters in generated secret")
                    .short('l')
                    .long("length")
                    .takes_value(true)
                    .default_value("40"),
            )
            .arg(
                clap::Arg::new("show")
                    .about("Print the generated secret instead of copying to clipboard")
                    .short('s')
                    .long("show"),
            )
            .arg(
                clap::Arg::new("alphanumeric")
                    .about("Use only alphanumeric characters")
                    .short('a')
                    .long("--alphanumeric"),
            )
    }
    fn repl_only(&self) -> bool {
        false
    }
}
