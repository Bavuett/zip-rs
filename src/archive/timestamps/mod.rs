pub mod last_modified_file_time;

mod implementation;

#[derive(Debug)]
pub struct Timestamps {
    last_modified_file_time: LastModifiedFileTime,
}
