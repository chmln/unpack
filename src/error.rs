use std::fmt::{Debug, Formatter};

#[derive(thiserror::Error)]
pub enum Error {
    #[error("Archives of type {0} are currently unsupported")]
    UnsupportedArchive(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Archive(#[from] compress_tools::Error),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
