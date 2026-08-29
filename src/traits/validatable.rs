use std::fs::File;
use std::io::{BufReader, Read};

use crate::utils::constants::ConstantValues;

pub trait Validatable {
    fn is_zip(&self) -> bool {
        false
    }

    fn is_zip_stream(&mut self) -> std::io::Result<bool> {
        Ok(false)
    }
}

impl Validatable for Vec<u8> {
    fn is_zip(&self) -> bool {
        if self.len() < 4 {
            return false;
        }

        self[0..4] == ConstantValues::ZIP_SIGNATURE
    }
}

impl Validatable for BufReader<File> {
    fn is_zip_stream(&mut self) -> std::io::Result<bool> {
        let mut buffer: [u8; 4] = [0; 4];
        let bytes_read: usize = self.read(&mut buffer)?;

        println!("Bytes read: {}.\nBuffer: {:?}", bytes_read, buffer);

        Ok(buffer == ConstantValues::ZIP_SIGNATURE)
    }
}
