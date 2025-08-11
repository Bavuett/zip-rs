use crate::{archive::flags::Flags, utils::constants::ConstantValues};

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

    pub fn central_directory_encryption(&self) -> bool {
        self.central_directory_encryption
    }

    pub fn set_central_directory_encryption(&mut self, buffer: &Vec<u8>) -> () {
        let raw_buffer: u16 = u16::from_le_bytes([buffer[0], buffer[1]]);
        self.central_directory_encryption = (raw_buffer & ConstantValues::FLAG_CENTRAL_DIRECTORY_ECNRYPTION) != 0;
    }

    pub fn encrypted(&self) -> bool {
        self.encrypted
    }

    pub fn set_encrypted(&mut self, buffer: &Vec<u8>) -> () {
        let raw_buffer: u16 = u16::from_le_bytes([buffer[0], buffer[1]]);
        self.encrypted = (raw_buffer & ConstantValues::FLAG_ENCRYPTED) != 0;
    }

    pub fn reduced_compression(&self) -> bool {
        self.reduced_compression
    }

    pub fn set_reduced_compression(&mut self, buffer: &Vec<u8>) -> () {
        let raw_buffer: u16 = u16::from_le_bytes([buffer[0], buffer[1]]);
        self.reduced_compression = (raw_buffer & ConstantValues::FLAG_REDUCED_COMPRESSION) != 0;
    }

    pub fn improved_compression(&self) -> bool {
        self.improved_compression
    }

    pub fn set_improved_compression(&mut self, buffer: &Vec<u8>) -> () {
        let raw_buffer: u16 = u16::from_le_bytes([buffer[0], buffer[1]]);
        self.improved_compression = (raw_buffer & ConstantValues::FLAG_IMPROVED_COMPRESSION) != 0;
    }

    pub fn values_in_data_descriptor(&self) -> bool {
        self.values_in_data_descriptor
    }

    pub fn set_values_in_data_descriptor(&mut self, buffer: &Vec<u8>) -> () {
        let raw_buffer: u16 = u16::from_le_bytes([buffer[0], buffer[1]]);
        self.values_in_data_descriptor = (raw_buffer & ConstantValues::FLAG_VALUES_IN_DATA_DESCRIPTOR) != 0;
    }

    pub fn enhanced_deflation(&self) -> bool {
        self.enhanced_deflation
    }

    pub fn set_enhanced_deflation(&mut self, buffer: &Vec<u8>) -> () {
        let raw_buffer: u16 = u16::from_le_bytes([buffer[0], buffer[1]]);
        self.enhanced_deflation = (raw_buffer & ConstantValues::FLAG_ENHANCED_DEFLATION) != 0;
    }

    pub fn patched_data_compression(&self) -> bool {
        self.patched_data_compression
    }

    pub fn set_patched_data_compression(&mut self, buffer: &Vec<u8>) -> () {
        let raw_buffer: u16 = u16::from_le_bytes([buffer[0], buffer[1]]);
        self.patched_data_compression = (raw_buffer & ConstantValues::FLAG_PATCHED_DATA_COMPRESSION) != 0;
    }

    pub fn strong_encryption(&self) -> bool {
        self.strong_encryption
    }

    pub fn set_strong_encryption(&mut self, buffer: &Vec<u8>) -> () {
        let raw_buffer: u16 = u16::from_le_bytes([buffer[0], buffer[1]]);
        self.strong_encryption = (raw_buffer & ConstantValues::FLAG_STRONG_ENCRYPTION) != 0;
    }

    pub fn utf8(&self) -> bool {
        self.utf8
    }

    pub fn set_utf8(&mut self, buffer: &Vec<u8>) -> () {
        let raw_buffer: u16 = u16::from_le_bytes([buffer[0], buffer[1]]);
        self.utf8 = (raw_buffer & ConstantValues::FLAG_UTF8) != 0;
    }
}