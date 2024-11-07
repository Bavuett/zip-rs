use crate::archive::flags::Flags;

pub struct Entry {
    pub offset: usize,
    pub bytes: Vec<u8>,
    pub flags: Flags,
}