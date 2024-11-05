use std::{fs::File, io::BufReader};

mod implementation;
mod entry;
pub mod flags;

pub struct Archive {
    file: BufReader<File>,

    pub name: String,
}