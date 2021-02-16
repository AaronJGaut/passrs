use passrs::cmd::{self, Command, Commands};
use passrs::cli;

use clap::App;

fn main() {
    let commands = cmd::CommandVec::build();

    let mut app = App::new("passrs")
        .version("0.1.0")
        .about("CLI password manager written in rust");

    for command in &commands {
        if !command.repl_only() {
            app = app.subcommand(command.clap_app());
        }
    }
        /*
        .subcommand(App::new("generate")
            .short_flag('G')
        )
        .subcommand(App::new("info")
            .short_flag('I')
        )
        .subcommand(App::new("list")
            .short_flag('L')
        )
        .subcommand(App::new("meta")
            .short_flag('M')
        )
        .subcommand(App::new("remove")
            .short_flag('R')
        )
        .subcommand(App::new("update")
            .short_flag('U')
        )
        .subcommand(App::new("write")
            .short_flag('W')
        )
        .subcommand(App::new("yank")
            .short_flag('Y')
        );
        */
    let matches = app.get_matches();

    let mut ran_command = false;
    for command in &commands {
        if let Some(cmd_matches) = matches.subcommand_matches(command.name()) {
            command.parse_and_run(cmd_matches);
            ran_command = true;
            break;
        }
    }
    if !ran_command {
        cli::repl(commands);
    }
}
