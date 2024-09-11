use std::{fs::File, io::BufReader};

mod implementation;
pub mod flags;

pub struct Archive {
    file: BufReader<File>,

    pub name: String,
}