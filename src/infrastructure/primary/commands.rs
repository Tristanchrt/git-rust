use crate::infrastructure::primary::command_handler::COMMAND;
use std::env;

pub fn cli_commands() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).and_then(|arg| COMMAND::from_str(arg)) {
        Some(COMMAND::COMMIT(data)) => {
            println!("{:?}", data(args));
        }
        Some(COMMAND::BRANCH(data)) => {
            println!("{:?}", data(args));
        }
        None => {
            println!("No valid command provided");
        }
    }
}
