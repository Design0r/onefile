use std::{
    env,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub inputs: Vec<PathBuf>,
}

impl Args {
    /// Converts any `./` into the absolute current directory.
    pub fn get_absolute_paths(&self) -> Vec<PathBuf> {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        self.inputs
            .iter()
            .map(|input| {
                if input == Path::new("./") {
                    return current_dir.clone();
                } else {
                    return input.canonicalize().unwrap().clone();
                }
            })
            .collect()
    }
}
