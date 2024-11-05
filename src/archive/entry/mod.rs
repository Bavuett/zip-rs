use crate::archive::flags::Flags;

pub struct Entry {
    pub offset: usize,
    pub flags: Flags,
}