use super::{Command, CommandWrapper};
use crate::db;
use rand::seq::IteratorRandom;

const UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
const NUM: &'static str = "0123456789";
const SPECIAL: &'static str = " `~!@#$%^&*()-_=+[]{};:'\"\\|,<.>/?";

pub struct ArgsGenerate {
    length: u32,
    punctuation: bool,
    show: bool,
}

fn generate_secret(length: u32, punctuation: bool) -> String {
    let mut char_set = String::new();
    char_set += UPPER;
    char_set += LOWER;
    char_set += NUM;
    if punctuation {
        char_set += SPECIAL;
    }
    let mut secret = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        secret.push(char_set.as_str().chars().choose(&mut rng).unwrap());
    }
    secret
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
        println!("{}", generate_secret(opts.length, opts.punctuation));
    }
    fn parse(&self, raw_args: &clap::ArgMatches, db: &mut db::Database) -> ArgsGenerate {
        ArgsGenerate {
            show: raw_args.is_present("show"),
            length: u32::from_str_radix(raw_args.value_of("length").unwrap(), 10).unwrap(),
            punctuation: raw_args.is_present("punctuation"),
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
                clap::Arg::new("punctuation")
                    .about("Use punctuation characters for extra entropy")
                    .short('p')
                    .long("--punctuation"),
            )
    }
    fn repl_only(&self) -> bool {
        false
    }
}
