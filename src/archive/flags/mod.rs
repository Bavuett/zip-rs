mod implementation;

#[derive(Debug)]
pub struct Flags {
    encrypted: bool,
    reduced_compression: bool,
    improved_compression: bool,
    values_in_data_descriptor: bool,
    enhanced_deflation: bool,
    patched_data_compression: bool,
    strong_encryption: bool,
    utf8: bool,
    central_directory_encryption: bool,
}