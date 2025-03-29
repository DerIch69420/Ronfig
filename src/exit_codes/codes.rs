pub enum ExitCode {
    // ExitCode for early termination
    EverythingFine = 0,

    // ErrorCodes for arguments
    InvalidArgument = 1,
    TooFewArguments = 2,
    TooManyArguments = 3,

    // ErrorCodes for not existing paths and files
    DirectoryDoesNotExist = 4,
    FileDoesNotExist = 5,
}

impl Into<i32> for ExitCode {
    fn into(self) -> i32 {
        self as i32
    }
}
