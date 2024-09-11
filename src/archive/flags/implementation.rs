use crate::archive::flags::Flags;

impl Flags {
    pub fn new() -> Self {
        Flags {
            central_directory_encryption: false,
            encrypted: false,
            reduced_compression: false,
            improved_compression: false,
            values_in_data_descriptor: false,
            enhanced_deflation: false,
            patched_data_compression: false,
            strong_encryption: false,
            utf8: false
        }
    }
}