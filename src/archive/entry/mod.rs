use crate::archive::flags::Flags;

mod implementation;

pub struct Entry {
    pub offset: usize,
    pub bytes: Vec<u8>,
    pub flags: Flags,
}
