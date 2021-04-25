mod cli;
mod error;

use cli::Cmd;
use error::Result;
use std::{
    fs::File,
    io::Cursor,
    path::{Path, PathBuf},
};

pub fn extract_archive(path: &Path) -> Result<PathBuf> {
    use memmap::Mmap;

    let target_path = {
        let mut parent_path = path.parent().unwrap().to_owned();
        parent_path.push(path.file_stem().unwrap());
        parent_path
    };

    compress_tools::uncompress_archive(
        Cursor::new(unsafe { Mmap::map(&File::open(path)?)? }),
        &target_path,
        compress_tools::Ownership::Ignore,
    )?;

    Ok(target_path)
}

fn main() -> Result<()> {
    use rayon::prelude::*;
    let opts: Cmd = argh::from_env();

    let extracted_paths = opts
        .files
        .par_iter()
        .map(|file| extract_archive(file))
        .collect::<Result<Vec<_>>>()?;

    if opts.files.len() == 1 && atty::isnt(atty::Stream::Stdout) {
        std::process::Command::new("xdg-open")
            .arg(&extracted_paths[0])
            .spawn()?
            .wait()?;
    } else {
        extracted_paths.iter().for_each(|path| println!("{}", path.display()));
    }

    Ok(())
}
