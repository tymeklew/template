use regex::Error as RegexError;

#[derive(Debug)]
pub enum Error {
    Regex(RegexError),
}

impl From<RegexError> for Error {
    fn from(value: RegexError) -> Self {
        Error::Regex(value)
    }
}
