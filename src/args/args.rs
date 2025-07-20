use std::env;

pub fn get_args() -> Result<Vec<String>, String> {
    // Get passed arguments
    let args: Vec<String> = env::args().collect();
    // args[0] -> location of binary
    // args[1] -> mode
    // args[2] -> additional argument

    // dbg!(&args);

    // No argument has been passed
    if args.len() == 1 {
        return Err(String::from("No arguments passed. Try adding \"help\""));
    }

    // Too many arguments
    if args.len() > 3 {
        return Err(String::from("Too many arguments passed. Try \"help\""));
    }

    return Ok(args);
}
