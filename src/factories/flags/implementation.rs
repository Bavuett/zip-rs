use crate::archive::flags::Flags;

use crate::factories::flags::FlagsFactory;

impl FlagsFactory {
    pub fn from(buffer: &Vec<u8>) -> std::io::Result<Flags> {
        if buffer.len() < 8 {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Buffer is not big enough. There should be at least 8 bytes."
                )
            );
        }

        Ok(Flags::new())
    }
}