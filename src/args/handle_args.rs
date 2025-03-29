use std::process::exit;

use super::super::exit_codes::codes::ExitCode;
use super::args::Mode;

pub fn handle_mode(args: Vec<String>) -> Mode {
    let mode = Mode::from_str(&args[1].to_lowercase());

    match mode {
        Mode::Help => help(),
        Mode::Copy => return Mode::Copy,
        Mode::Indvalid => invalid(),
    }
}

fn help() -> ! {
    println!("Check out the repo for more information");
    println!("https://github.com/DerIch69420/Ronfig");
    exit(ExitCode::EverythingFine.into());
}

fn invalid() -> ! {
    println!("Unknown argument!");
    println!("Try using \"help\"");
    exit(ExitCode::InvalidArgument.into());
}
