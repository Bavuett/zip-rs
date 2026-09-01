use crate::archive::{compression_method::CompressionMethod, flags::Flags};

mod implementation;

pub struct Entry {
    pub offset: usize,
    pub bytes: Vec<u8>,
    pub flags: Flags,
    pub compression_method: CompressionMethod,
}
