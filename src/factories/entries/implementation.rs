use crate::archive::{entry::Entry, flags::Flags};
use crate::factories::entries::EntriesFactory;

use std::io::{BufReader, Read, Seek, SeekFrom};
use std::fs::File;

impl EntriesFactory {
    pub fn from(file: &mut BufReader<File>, offset: usize, next_offset: usize) -> Result<Entry, std::io::Error> {
        _ = match file.seek(SeekFrom::Start(offset as u64)) {
            Ok(_) => (),
            Err(error) => return Err(error)
        };
        
        println!("\n \n[GENERATING ENTRY]\nOffset: {}, Next Offset: {}, Operation: {:?}", offset, next_offset, next_offset - offset);
        
        let mut buffer: Vec<u8> = vec![0; next_offset - offset];

        println!("Vec has been declared with capacity: {}", buffer.len());

        _ = match file.read(&mut buffer) {
            Ok(_) => (),
            Err(error) => return Err(error)
        };

        println!("Buffer inside Entries Factory: {:?}", buffer);

        Ok(Entry {
            offset,
            bytes: buffer,
            flags: Flags::new(),
        })
    }
}
