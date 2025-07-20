mod args;
mod config;
mod config_file;
mod modes;

use std::error::Error;

use args::{args::get_args, handle_args::handle_mode};

use modes::copy::copy::copy;
use modes::help::help::help;
use modes::invalid::invalid::invalid;
use modes::modes::Mode;
use modes::new::new::new;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = match get_args() {
        Ok(args) => args,
        Err(err_msg) => return Err(err_msg.into()),
    };
    let mode: Mode = handle_mode(&args[1]);

    if mode == Mode::Invalid {
        invalid();
    }

    if mode == Mode::Help {
        help();
    }

    if mode == Mode::New {
        if args.len() != 3 {
            return Err("Wrong amount of arguments".into());
        }

        if let Err(e) = new(&args) {
            return Err(e);
        }
    }

    if mode == Mode::Copy {
        if args.len() != 2 && args.len() != 3 {
            return Err("Wrong amount of arguments".into());
        }

        copy(&args);
    }

    Ok(())
}
