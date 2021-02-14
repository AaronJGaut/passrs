use rustyline::error::ReadlineError;
use rustyline::Editor;

use passrs::confirm_interrupt;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("\n\x1b[92;1mpassrs>\x1b[0m ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Processing...Done");
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                if confirm_interrupt() {
                    break
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
