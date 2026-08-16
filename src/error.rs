use std::io;

#[derive(thiserror::Error, Debug)]
pub enum UebersetzError {
    #[error("filesystem type error: {0}")]
    IO(String),
    #[error("tera error: {0}")]
    Tera(String),
}

impl From<UebersetzError> for io::Error {
    fn from(err: UebersetzError) -> Self {
        std::io::Error::other(err)
    }
}

impl From<io::Error> for UebersetzError {
    fn from(err: io::Error) -> Self {
        Self::IO(err.to_string())
    }
}

impl From<tera::Error> for UebersetzError {
    fn from(err: tera::Error) -> Self {
        Self::Tera(err.to_string())
    }
}
