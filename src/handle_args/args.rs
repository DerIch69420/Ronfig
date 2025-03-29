use std::env;
use std::process;

pub fn handle_args() -> Vec<String> {
    // Get passed arguments
    let args: Vec<String> = env::args().collect();
    // args[0] -> location of binary
    // args[1] -> first argument
    // dbg!(&args);

    if args.len() == 1 {
        println!("No arguments passed");
        process::exit(1);
    }

    return args;
}
