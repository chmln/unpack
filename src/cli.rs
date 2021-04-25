use argh::FromArgs;
use std::path::PathBuf;

#[derive(FromArgs)]
/// Unpack archives
pub struct Cmd {
    #[argh(positional)]
    /// archives
    pub files: Vec<PathBuf>,
}
