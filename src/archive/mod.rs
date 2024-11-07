use std::{fs::File, io::BufReader};

use entry::Entry;

mod implementation;

pub mod entry;
pub mod flags;

pub struct Archive {
    file: BufReader<File>,
    pub entries: Vec<Entry>,
}