use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocErrors {
    #[error("Could not parse input: {0}")]
    ParseError(String),
}

