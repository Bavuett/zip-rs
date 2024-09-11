mod implementation;

pub struct Flags {
    pub encrypted: bool,
    pub reduced_compression: bool,
    pub improved_compression: bool,
    pub values_in_data_descriptor: bool,
    pub enhanced_deflation: bool,
    pub patched_data_compression: bool,
    pub strong_encryption: bool,
    pub utf8: bool,
    pub central_directory_encryption: bool,
}