use crate::archive::flags::Flags;
use crate::factories::flags::FlagsFactory;

impl FlagsFactory {
    pub fn from(buffer: &Vec<u8>) -> Result<Flags, std::io::Error> {
        let mut flags: Flags =  Flags::new();

        flags.set_central_directory_encryption(&buffer)?;
        flags.set_encrypted(&buffer)?;
        flags.set_improved_compression(&buffer)?;
        flags.set_values_in_data_descriptor(&buffer)?;
        flags.set_enhanced_deflation(&buffer)?;
        flags.set_patched_data_compression(&buffer)?;
        flags.set_strong_encryption(&buffer)?;
        flags.set_utf8(&buffer)?;

        println!(
            "Flags as u16: [{:?}], which equals to Bytes: [{:?}].\nAs Data Structure: [{:?}]", 
            flags.as_u16_le(), format!("{:016b}", flags.as_u16_le()), flags
        );

        Ok(flags)
    }
}