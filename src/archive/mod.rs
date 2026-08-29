use std::{fs::File, io::BufReader};

use entry::Entry;

mod implementation;
pub(crate) mod local_file_headers_offsets;

pub mod compression_method;
pub mod entry;
pub mod flags;

pub struct Archive {
    file: BufReader<File>,
    pub entries: Vec<Entry>,
}
