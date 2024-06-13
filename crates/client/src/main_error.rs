use crate::parser::ParseError;

#[derive(Debug)]
pub enum MainError {
    Parse(ParseError),
    Bincode(bincode::Error),
    Command,
}

impl From<ParseError> for MainError {
    fn from(value: ParseError) -> Self {
        MainError::Parse(value)
    }
}

impl From<bincode::Error> for MainError {
    fn from(value: bincode::Error) -> Self {
        MainError::Bincode(value)
    }
}
