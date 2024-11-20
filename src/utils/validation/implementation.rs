use crate::utils::validation::ValidationUtilities;
use crate::utils::constants::ConstantValues;

use std::fs::File;
use std::io::{BufReader, Read};

impl ValidationUtilities {
    pub fn is_zip_file(file: &mut BufReader<File>) -> std::io::Result<bool> {
        let mut buffer: [u8; 4] = [0; 4];
        let bytes_read: usize = match file.read(&mut buffer) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        println!("Bytes read: {}.\nBuffer: {:?}", bytes_read, buffer);

        if buffer == ConstantValues::ZIP_SIGNATURE {
            return Ok(true);
        }

        Ok(false)
    }

    pub fn is_zip_entry(buffer: &Vec<u8>) -> bool {    
        if buffer.len() < 4 {
            return false;
        };
        
        if buffer[0..4] == ConstantValues::ZIP_SIGNATURE {
            return true;
        }

        false
    }
}
