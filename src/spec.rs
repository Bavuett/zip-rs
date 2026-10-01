pub struct LocalFileHeader<'a> {
    pub version_needed: u16,
    pub general_purpose_bit_flags: u16,
    pub compression_method: u16,
    pub file_modification_time: u16,
    pub file_modification_date: u16,
    pub checksum_crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
    pub file_name: &'a str,
}

pub struct EndOfCentralDirectory<'a> {
    pub disk_number: u16,
    pub disk_with_central_directory: u16,
    pub total_entries_on_this_disk: u16,
    pub total_entries: u16,
    pub size_of_central_directory: u32,
    pub start_of_central_directory_offset: u32,
    pub archive_comment_length: u16,
    pub archive_comment: &'a str,
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
