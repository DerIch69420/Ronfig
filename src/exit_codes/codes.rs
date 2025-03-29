pub enum ExitCode {
    EverythingFine = 0,
    InvalidArgument = 1,
    TooFewArguments = 2,
    TooManyArguments = 3,
}

impl Into<i32> for ExitCode {
    fn into(self) -> i32 {
        self as i32
    }
}
