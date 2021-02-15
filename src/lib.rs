use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;

use std::error::Error;
use std::io;

use clap::App;

const command_names: &'static [&'static str] = &["add", "clear", "help", "generate", "info", "list", "meta", "quit", "remove", "write", "yank"];

pub trait CommandWrapper {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn parse_and_run(&self);
}

impl<C: Command> CommandWrapper for C {
    fn name(&self) -> &'static str {
        self.name()
    }
    fn help(&self) -> &'static str {
        self.help()
    }
    fn parse_and_run(&self) {
        self.run(self.parse())
    }
}

pub trait Command {
    type Args;
    fn new() -> Box<dyn CommandWrapper>;
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn run(&self, options: Self::Args);
    fn parse(&self) -> Self::Args;
}

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
    fn parse(&self) -> ArgsAdd {
        ArgsAdd { show: true }
    }
}


pub fn confirm_interrupt() -> bool {
    println!("Press ctrl-c or ctrl-d again to quit or any key to continue");
    // need raw mode to read a single character
    match get_key() {
        Key::Ctrl('c') | Key::Ctrl('d') => true,
        _ => false,
    }
}

pub fn to_args(input: &str) -> Vec::<&str> {
    input.split_ascii_whitespace().collect()
}

pub fn get_key() -> Key {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    io::stdin().keys().next().unwrap().unwrap()
}

pub fn match_command(input: &str) -> Result<&'static str, &'static str> {
    for name in command_names {
        if let Some(i) = name.find(input) {
            if i == 0 {
                return Ok(name);
            }
        }
    }
    Err("Unknown command")
}


pub fn edit(prompt: &str) -> String
{
    String::from("")
}

pub enum DbEncryption {
    Fernet,
}

pub enum DbFormat {
    Json,
}

pub struct Database {
    encryption: DbEncryption,
    format: DbFormat,
    path: String,
    records: Option<Vec<Record>>,  // Lazy load only when needed
}

pub struct Record {
    account: String,
    username: Option<String>,
    password: String,
    notes: Option<String>,
}

pub fn cmd_add() {
    println!("add")
}

pub struct GenerateOpts {
    length: u32,
    alphanumeric: bool,
    show: bool,
}

pub fn cmd_generate() {
    println!("generate")
}

pub struct InfoOpts {
    show: bool,
    index: u32,
}

pub fn cmd_info() {
    println!("info")
}

pub struct ListOpts {
}

pub fn cmd_list() {
    println!("list")
}

pub struct MetaOpts {
    show: bool,
}

pub fn cmd_meta() {
    println!("meta")
}

pub struct RemoveOpts {
    index: u32,
}

pub fn cmd_remove() {
    println!("remove")
}

pub struct UpdateOpts {
    account: bool,
    username: bool,
    password: bool,
    notes: bool,
    show: bool,
    index: u32,
}

pub fn cmd_update() {
    println!("update")
}

pub struct WriteOpts {
    encryption: DbEncryption,
    format: DbFormat,
    password: bool,
    show: bool,
    filepath: String,
}

pub fn cmd_write() {
    println!("write")
}

pub struct YankOpts {
    index: u32,
}

pub fn cmd_yank() {
    println!("yank")
}
