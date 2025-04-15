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

