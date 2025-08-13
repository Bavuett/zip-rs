use crate::utils::constants::ConstantValues;

impl ConstantValues {
    pub const ZIP_SIGNATURE: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];

    pub const FLAG_ENCRYPTED: u16                       = 0b0000_0000_0000_0001; // Bit 0 toggled.
    pub const FLAG_REDUCED_COMPRESSION: u16             = 0b0000_0000_0000_0010; // Bit 1 toggled.
    pub const FLAG_IMPROVED_COMPRESSION: u16            = 0b0000_0000_0000_0100; // Bit 2 toggled.
    pub const FLAG_VALUES_IN_DATA_DESCRIPTOR: u16       = 0b0000_0000_0000_1000; // Bit 3 toggled.
    pub const FLAG_ENHANCED_DEFLATION: u16              = 0b0000_0000_0001_0000; // Bit 4 toggled.
    pub const FLAG_PATCHED_DATA_COMPRESSION: u16        = 0b0000_0000_0010_0000; // Bit 5 toggled.
    pub const FLAG_STRONG_ENCRYPTION: u16               = 0b0000_0000_0100_0000; // Bit 6 toggled.
    pub const FLAG_UTF8: u16                            = 0b0000_1000_0000_0000; // Bit 11 toggled.
    pub const FLAG_CENTRAL_DIRECTORY_ENCRYPTION: u16    = 0b0010_0000_0000_0000; // Bit 13 toggled.
}