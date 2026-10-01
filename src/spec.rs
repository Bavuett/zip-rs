pub struct LocalFileHeader<'a> {
    version_needed: u16,
    general_purpose_bit_flags: u16,
    compression_method: u16,
    file_modification_time: u16,
    file_modification_date: u16,
    checksum_crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    file_name_length: u16,
    extra_field_length: u16,
    file_name: &'a str,
}

impl<'a> LocalFileHeader<'a> {
    pub fn new(
        version_needed: u16,
        general_purpose_bit_flags: u16,
        compression_method: u16,
        file_modification_time: u16,
        file_modification_date: u16,
        checksum_crc32: u32,
        compressed_size: u32,
        uncompressed_size: u32,
        file_name_length: u16,
        extra_field_length: u16,
        file_name: &'a str,
    ) -> Self {
        Self {
            version_needed,
            general_purpose_bit_flags,
            compression_method,
            file_modification_date,
            file_modification_time,
            checksum_crc32,
            compressed_size,
            uncompressed_size,
            file_name_length,
            extra_field_length,
            file_name,
        }
    }
}
