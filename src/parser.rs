use crate::{
    error::ZipError,
    reader::Reader,
    spec::{EndOfCentralDirectory, LocalFileHeader},
};

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
        let file_size: usize = self.reader.length();

        // A ZIP File must be at least 22 bytes in size.
        if file_size < 22 {
            return Ok(false);
        }

        match self.get_end_of_central_directory() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn get_local_file_header(&mut self) -> Result<LocalFileHeader<'a>, ZipError> {
        self.reader.seek(0)?;

        self.reader.expect_signature(0x04034b50)?;

        let version_needed: u16 = self.reader.read_u16_le()?;
        let general_purpose_bit_flags: u16 = self.reader.read_u16_le()?;
        let compression_method: u16 = self.reader.read_u16_le()?;
        let file_modification_time: u16 = self.reader.read_u16_le()?;
        let file_modification_date: u16 = self.reader.read_u16_le()?;
        let checksum_crc32: u32 = self.reader.read_u32_le()?;
        let compressed_size: u32 = self.reader.read_u32_le()?;
        let uncompressed_size: u32 = self.reader.read_u32_le()?;
        let file_name_length: u16 = self.reader.read_u16_le()?;
        let extra_field_length: u16 = self.reader.read_u16_le()?;

        let file_name_bytes: &[u8] = self.reader.read_bytes(file_name_length as usize)?;
        let file_name: &str = match std::str::from_utf8(file_name_bytes) {
            Ok(file_name) => file_name,
            Err(_) => return Err(ZipError::InvalidUtf8),
        };

        let local_file_header: LocalFileHeader<'a> = LocalFileHeader::new(
            version_needed,
            general_purpose_bit_flags,
            compression_method,
            file_modification_time,
            file_modification_date,
            checksum_crc32,
            compressed_size,
            uncompressed_size,
            file_name_length,
            extra_field_length,
            file_name,
        );

        Ok(local_file_header)
    }

    pub fn get_end_of_central_directory(&mut self) -> Result<EndOfCentralDirectory<'a>, ZipError> {
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
                let disk_number: u16 = self.reader.read_u16_le()?;
                let disk_with_central_directory: u16 = self.reader.read_u16_le()?;
                let total_entries_on_this_disk: u16 = self.reader.read_u16_le()?;
                let total_entries_in_central_directory: u16 = self.reader.read_u16_le()?;
                let size_of_central_directory: u32 = self.reader.read_u32_le()?;
                let start_of_central_directory_offset: u32 = self.reader.read_u32_le()?;
                let archive_comment_length: u16 = self.reader.read_u16_le()?;

                let archive_comments_bytes: &[u8] =
                    self.reader.read_bytes(archive_comment_length as usize)?;
                let archive_comment: &'a str = match std::str::from_utf8(archive_comments_bytes) {
                    Ok(archive_comment) => archive_comment,
                    Err(_) => return Err(ZipError::InvalidUtf8),
                };

                return Ok(EndOfCentralDirectory {
                    disk_number,
                    disk_with_central_directory,
                    total_entries_on_this_disk,
                    total_entries_in_central_directory,
                    size_of_central_directory,
                    start_of_central_directory_offset,
                    archive_comment_length,
                    archive_comment,
                });
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
