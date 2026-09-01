mod implementation;
mod last_modified_file_time;

use crate::archive::timestamps::last_modified_file_time::LastModifiedFileTime;

#[derive(Debug)]
pub struct Timestamps {
    last_modified_file_time: LastModifiedFileTime,
}
