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
            file_modification_time,
            file_modification_date,
            checksum_crc32,
            compressed_size,
            uncompressed_size,
            file_name_length,
            extra_field_length,
            file_name,
        }
    }
}

pub struct CentralDirectory<'a> {
    pub version_made_by: u16,
    pub version_needed: u16,
    pub general_purpose_bit_flags: u16,
    pub compression_method: u16,
    pub file_modification_time: u16,
    pub file_modification_date: u16,
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
    pub file_comment_length: u16,
    pub disk_number_start: u16,
    pub internal_file_attributes: u16,
    pub external_file_attributes: u16,
    pub relative_offset_of_local_header: u32,
    pub file_name: &'a str,
    pub extra_field: &'a [u8],
    pub file_comment: &'a str,
}

pub struct EndOfCentralDirectory<'a> {
    pub disk_number: u16,
    pub disk_with_central_directory: u16,
    pub total_entries_on_this_disk: u16,
    pub total_entries_in_central_directory: u16,
    pub size_of_central_directory: u32,
    pub start_of_central_directory_offset: u32,
    pub archive_comment_length: u16,
    pub archive_comment: &'a str,
}

impl<'a> EndOfCentralDirectory<'a> {
    pub fn new(
        disk_number: u16,
        disk_with_central_directory: u16,
        total_entries_on_this_disk: u16,
        total_entries_in_central_directory: u16,
        size_of_central_directory: u32,
        start_of_central_directory_offset: u32,
        archive_comment_length: u16,
        archive_comment: &'a str,
    ) -> Self {
        Self {
            disk_number,
            disk_with_central_directory,
            total_entries_on_this_disk,
            total_entries_in_central_directory,
            size_of_central_directory,
            start_of_central_directory_offset,
            archive_comment_length,
            archive_comment,
        }
    }
}
