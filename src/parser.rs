use crate::{error::ZipError, reader::Reader};

pub struct Parser<'a> {
    pub reader: Reader<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            reader: Reader::new(bytes),
        }
    }

    pub fn is_valid_zip(&mut self) -> Result<bool, ZipError> {
        let file_size = self.reader.length();

        // A ZIP File must be at least 22 bytes in size.
        if file_size < 22 {
            return Ok(false);
        }

        // A ZIP File may have a final comment that is max 65535 bytes long (0xFFFF).
        // So we know that the signature is has to be located in the last 65KB, more
        // or less.
        let max_offset: usize = 22 + 0xFFFF;

        let search_limit: usize = if file_size > max_offset {
            file_size - max_offset
        } else {
            // the file is smaller than 65557 (655535 + 22) bytes, se we scan until byte zero.
            0
        };

        self.reader.expect_signature(0x04034b50)?;

        // We put = before `(file_size - 22)` to let the loop use the inclusive range.
        // Rust default to exlusive range.
        for pos in (search_limit..=(file_size - 22)).rev() {
            self.reader.seek(pos)?;

            let signature: u32 = self.reader.read_u32_le()?;

            // We found the End of Central Directory signature. The `reader` cursor is positioned right after
            // the 4 bytes of the signature, ready to read the rest of the EOCD.
            if signature == 0x06054b50 {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn find_eocd(&mut self) -> Result<(), ZipError> {
        let file_size = self.reader.length();

        let max_offset: usize = 22 + 0xFFFF;

        let search_limit: usize = if file_size > max_offset {
            file_size - max_offset
        } else {
            0
        };

        for pos in (search_limit..=(file_size - 22)).rev() {
            self.reader.seek(pos)?;

            let signature: u32 = self.reader.read_u32_le()?;

            if signature == 0x06054b50 {
                return Ok(());
            }
        }

        // Didn't find anyting. Let's use BadSignature.
        Err(ZipError::BadSignature {
            offset: 0,
            expected: 0x06054b50,
            got: 0,
        })
    }
}
