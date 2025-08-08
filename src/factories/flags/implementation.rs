use crate::archive::flags::Flags;
use crate::factories::flags::FlagsFactory;

impl FlagsFactory {
    pub fn from(buffer: &Vec<u8>) -> Result<Flags, std::io::Error> {
        let flags_as_binary_string: String = format!("{:016b}", buffer[4]);
        let flags_as_binary_chars: std::str::Chars<'_> = flags_as_binary_string.chars();

        println!("Flags as Byte in String: {:?}", flags_as_binary_string);
        println!("Flags as Byte in Chars: {:?}", flags_as_binary_chars);

        let mut flags_as_binary: Vec<i8> = Vec::new();
        
        flags_as_binary_chars.for_each(|item: char| flags_as_binary.push(item as i8 - 48));

        println!("Flags as Binary as i8: {:?}", flags_as_binary);
        Ok(Flags::new())
    }
}