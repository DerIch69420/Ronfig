use std::{env, process::exit};

pub fn get_args() -> Vec<String> {
    // Get passed arguments
    let args: Vec<String> = env::args().collect();
    // args[0] -> location of binary
    // args[1] -> mode
    // args[2] -> additional argument

    // dbg!(&args);

    if args.len() == 1 {
        println!("No arguments passed");
        println!("Try adding \"help\"");
        exit(1);
    }

    return args;
}

// Enum for handling the Mode of the application
// For the first argument passed after binary
#[derive(PartialEq)]
pub enum Mode {
    Help,
    Copy,
    Indvalid,
}

impl Mode {
    pub fn from_str(arg: &str) -> Self {
        match arg {
            "help" => Mode::Help,
            "copy" => Mode::Copy,
            _ => Mode::Indvalid,
        }
    }
}
