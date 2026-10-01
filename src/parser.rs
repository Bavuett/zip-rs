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

        match self.get_end_of_central_directory() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn get_end_of_central_directory(&mut self) -> Result<(), ZipError> {
        let file_size = self.reader.length();

        // A ZIP File may have a final comment that is max 65535 bytes long (0xFFFF).
        // So we know that the signature is has to be located in the last 65KB, more
        // or less.
        let max_offset: usize = 22 + 0xFFFF;

        let search_limit: usize = if file_size > max_offset {
            file_size - max_offset
        } else {
            // The file is smaller than 65557 (655535 + 22) bytes, se we scan until byte zero.
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

    pub fn get_version(&mut self) -> Result<u16, ZipError> {
        self.reader.seek(0)?;

        // Expecting a Signature sends us forward 4 bytes. We may read the version.
        self.reader.expect_signature(0x04034b50)?;

        let version: u16 = self.reader.read_u16_le()?;

        Ok(version)
    }

    pub fn get_general_purpose_bit_flags(&mut self) -> Result<u16, ZipError> {
        self.reader.seek(0)?;

        self.reader.expect_signature(0x04034b50)?;

        self.reader.skip(2)?;

        let general_purpose_bit_flags: u16 = self.reader.read_u16_le()?;

        Ok(general_purpose_bit_flags)
    }

    pub fn get_compression_method(&mut self) -> Result<u16, ZipError> {
        self.reader.seek(0)?;

        self.reader.expect_signature(0x04034b50)?;

        self.reader.skip(4)?;

        let compression_method: u16 = self.reader.read_u16_le()?;

        Ok(compression_method)
    }

    pub fn get_file_modification_time(&mut self) -> Result<u16, ZipError> {
        self.reader.seek(0)?;

        self.reader.expect_signature(0x04034b50)?;

        self.reader.skip(6)?;

        let file_modification_time: u16 = self.reader.read_u16_le()?;

        Ok(file_modification_time)
    }

    pub fn get_file_modification_date(&mut self) -> Result<u16, ZipError> {
        self.reader.seek(0)?;

        self.reader.expect_signature(0x04034b50)?;

        self.reader.skip(8)?;

        let file_modification_date: u16 = self.reader.read_u16_le()?;

        Ok(file_modification_date)
    }
}
