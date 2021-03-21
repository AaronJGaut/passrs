extern crate copypasta;

use super::{Command, CommandWrapper};
use crate::db;
use copypasta::{ClipboardContext, ClipboardProvider};
use rand::seq::IteratorRandom;
use crate::error::PassError;

const UPPER: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &'static str = "abcdefghijklmnopqrstuvwxyz";
const NUM: &'static str = "0123456789";
const SPECIAL: &'static str = "`~!@#$%^&*()-_=+[]{};:'\"\\|,<.>/?";

pub struct ArgsGenerate {
    length: u32,
    punctuation: bool,
    yank: bool,
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
        "Randomly generate a secret"
    }
    fn run(&self, opts: ArgsGenerate, db: &mut db::Database) -> Result<(), PassError> {
        let secret = generate_secret(opts.length, opts.punctuation);
        if opts.yank {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(secret);
            println!("Secret copied to clipboard.");
        } else {
            println!("{}", secret);
        }
        Ok(())
    }
    fn parse(
        &self,
        raw_args: &clap::ArgMatches,
        db: &mut db::Database,
    ) -> Result<ArgsGenerate, PassError> {
        let length_str = raw_args.value_of("length").unwrap();
        let length = match u32::from_str_radix(length_str, 10) {
            Ok(length) => length,
            Err(_) => return Err(PassError::Other(format!("Failed to parse length \"{}\"", length_str))),
        };
        Ok(ArgsGenerate {
            yank: raw_args.is_present("yank"),
            length: length,
            punctuation: raw_args.is_present("punctuation"),
        })
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
                clap::Arg::new("yank")
                    .about("Copy the generated secret to clipboard instead of printing")
                    .short('y')
                    .long("yank"),
            )
            .arg(
                clap::Arg::new("punctuation")
                    .about("Use punctuation characters for extra entropy")
                    .short('p')
                    .long("punctuation"),
            )
    }
    fn repl_only(&self) -> bool {
        false
    }
}
