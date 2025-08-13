use crate::{
    archive::flags::Flags, traits::validatable::Validatable, utils::constants::ConstantValues,
};

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
            utf8: false,
        }
    }

    pub fn as_u16_le(&self) -> u16 {
        let flags_as_u16: u16;

        let central_directory_encryption: u16 = 
            if self.central_directory_encryption { ConstantValues::FLAG_CENTRAL_DIRECTORY_ENCRYPTION } else { 0 };
        let encrypted: u16 = 
            if self.encrypted { ConstantValues::FLAG_ENCRYPTED } else { 0 };
        let reduced_compression: u16 =
            if self.reduced_compression { ConstantValues::FLAG_REDUCED_COMPRESSION } else { 0 };
        let improved_compression: u16 = 
            if self.improved_compression { ConstantValues::FLAG_IMPROVED_COMPRESSION } else { 0 };
        let values_in_data_descriptor: u16 = 
            if self.values_in_data_descriptor { ConstantValues::FLAG_VALUES_IN_DATA_DESCRIPTOR } else { 0 };
        let enhanced_deflation: u16 =
            if self.enhanced_deflation { ConstantValues::FLAG_ENHANCED_DEFLATION } else { 0 };
        let patched_data_compression: u16 =
            if self.patched_data_compression { ConstantValues::FLAG_PATCHED_DATA_COMPRESSION } else { 0 };
        let strong_encryption: u16 = 
            if self.strong_encryption { ConstantValues::FLAG_STRONG_ENCRYPTION } else { 0 };
        let utf8: u16 = 
            if self.utf8 { ConstantValues::FLAG_UTF8 } else { 0 };
        

        flags_as_u16 = 
            central_directory_encryption + 
            encrypted + 
            reduced_compression + 
            improved_compression + 
            values_in_data_descriptor + 
            enhanced_deflation + 
            patched_data_compression +
            strong_encryption + 
            utf8;
        
        flags_as_u16
    }

    pub fn central_directory_encryption(&self) -> bool {
        self.central_directory_encryption
    }

    pub fn set_central_directory_encryption(
        &mut self,
        buffer: &Vec<u8>,
    ) -> Result<(), std::io::Error> {
        if !(buffer.is_zip()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not a valid zip.",
            ));
        };

        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not big enough to determine its General Purpose Flags",
            ));
        };

        let raw_buffer: u16 = u16::from_le_bytes([buffer[6], buffer[7]]);
        Ok(self.central_directory_encryption =
            (raw_buffer & ConstantValues::FLAG_CENTRAL_DIRECTORY_ENCRYPTION) != 0)
    }

    pub fn encrypted(&self) -> bool {
        self.encrypted
    }

    pub fn set_encrypted(&mut self, buffer: &Vec<u8>) -> Result<(), std::io::Error> {
        if !buffer.is_zip() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not a valid zip.",
            ));
        };

        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not big enough to determine its General Purpose Flags",
            ));
        };

        let raw_buffer: u16 = u16::from_le_bytes([buffer[6], buffer[7]]);
        Ok(self.encrypted = (raw_buffer & ConstantValues::FLAG_ENCRYPTED) != 0)
    }

    pub fn reduced_compression(&self) -> bool {
        self.reduced_compression
    }

    pub fn set_reduced_compression(&mut self, buffer: &Vec<u8>) -> Result<(), std::io::Error> {
        if !buffer.is_zip() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not a valid zip.",
            ));
        };

        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not big enough to determine its General Purpose Flags",
            ));
        };

        let raw_buffer: u16 = u16::from_le_bytes([buffer[6], buffer[7]]);
        Ok(self.reduced_compression = (raw_buffer & ConstantValues::FLAG_REDUCED_COMPRESSION) != 0)
    }

    pub fn improved_compression(&self) -> bool {
        self.improved_compression
    }

    pub fn set_improved_compression(&mut self, buffer: &Vec<u8>) -> Result<(), std::io::Error> {
        if !buffer.is_zip() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not a valid zip.",
            ));
        };

        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not big enough to determine its General Purpose Flags",
            ));
        };

        let raw_buffer: u16 = u16::from_le_bytes([buffer[6], buffer[7]]);
        Ok(self.improved_compression =
            (raw_buffer & ConstantValues::FLAG_IMPROVED_COMPRESSION) != 0)
    }

    pub fn values_in_data_descriptor(&self) -> bool {
        self.values_in_data_descriptor
    }

    pub fn set_values_in_data_descriptor(
        &mut self,
        buffer: &Vec<u8>,
    ) -> Result<(), std::io::Error> {
        if !buffer.is_zip() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not a valid zip.",
            ));
        };

        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not big enough to determine its General Purpose Flags",
            ));
        };

        let raw_buffer: u16 = u16::from_le_bytes([buffer[6], buffer[7]]);
        Ok(self.values_in_data_descriptor =
            (raw_buffer & ConstantValues::FLAG_VALUES_IN_DATA_DESCRIPTOR) != 0)
    }

    pub fn enhanced_deflation(&self) -> bool {
        self.enhanced_deflation
    }

    pub fn set_enhanced_deflation(&mut self, buffer: &Vec<u8>) -> Result<(), std::io::Error> {
        if !buffer.is_zip() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not a valid zip.",
            ));
        };

        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not big enough to determine its General Purpose Flags",
            ));
        };

        let raw_buffer: u16 = u16::from_le_bytes([buffer[6], buffer[7]]);
        Ok(self.enhanced_deflation = (raw_buffer & ConstantValues::FLAG_ENHANCED_DEFLATION) != 0)
    }

    pub fn patched_data_compression(&self) -> bool {
        self.patched_data_compression
    }

    pub fn set_patched_data_compression(&mut self, buffer: &Vec<u8>) -> Result<(), std::io::Error> {
        if !buffer.is_zip() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not a valid zip.",
            ));
        };

        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not big enough to determine its General Purpose Flags",
            ));
        };

        let raw_buffer: u16 = u16::from_le_bytes([buffer[6], buffer[7]]);
        Ok(self.patched_data_compression =
            (raw_buffer & ConstantValues::FLAG_PATCHED_DATA_COMPRESSION) != 0)
    }

    pub fn strong_encryption(&self) -> bool {
        self.strong_encryption
    }

    pub fn set_strong_encryption(&mut self, buffer: &Vec<u8>) -> Result<(), std::io::Error> {
        if !buffer.is_zip() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not a valid zip.",
            ));
        };

        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not big enough to determine its General Purpose Flags",
            ));
        };
        
        let raw_buffer: u16 = u16::from_le_bytes([buffer[6], buffer[7]]);
        Ok(self.strong_encryption = (raw_buffer & ConstantValues::FLAG_STRONG_ENCRYPTION) != 0)
    }

    pub fn utf8(&self) -> bool {
        self.utf8
    }

    pub fn set_utf8(&mut self, buffer: &Vec<u8>) -> Result<(), std::io::Error> {
        if !buffer.is_zip() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not a valid zip.",
            ));
        };

        if buffer.len() < 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is not big enough to determine its General Purpose Flags",
            ));
        };

        let raw_buffer: u16 = u16::from_le_bytes([buffer[6], buffer[7]]);
        Ok(self.utf8 = (raw_buffer & ConstantValues::FLAG_UTF8) != 0)
    }
}
