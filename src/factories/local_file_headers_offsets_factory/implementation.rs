use crate::factories::local_file_headers_offsets_factory::LocalFileHeadersOffsetsFactory;

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

impl LocalFileHeadersOffsetsFactory {
    pub fn from(file: &mut BufReader<File>, size: u64) -> Result<Vec<usize>, std::io::Error> {
        let mut local_file_headers_offsets: Vec<usize> = Vec::new();
        let mut check_vec: Vec<u8> = Vec::new();
    
        let mut buffer: [u8; 256] = [0; 256];
        let mut counter: usize = 0;
    
        _ = match file.seek(SeekFrom::Start(0)) {
            Ok(_) => (),
            Err(error) => return Err(error),
        };
    
        while counter < size as usize {
            _ = match file.read(&mut buffer) {
                Ok(result) => result,
                Err(error) => return Err(error),
            };
    
            println!("Current buffer: {:?}", buffer);
    
            for i in 0..buffer.len() {
                match buffer[i] {
                    0x50 => {
                        check_vec.clear();
                        check_vec.push(0x50);
                    }
                    0x4B => {
                        if check_vec == [0x50] {
                            check_vec.push(0x4B);
                        } else {
                            check_vec.clear();
                        }
                    }
                    0x03 => {
                        if check_vec == [0x50, 0x4B] {
                            check_vec.push(0x03);
                        } else {
                            check_vec.clear();
                        }
                    }
                    0x04 => {
                        if check_vec == [0x50, 0x4B, 0x03] {
                            check_vec.push(0x04);
                        } else {
                            check_vec.clear();
                        }
                    }
                    _ => check_vec.clear(),
                }

                
    
                if check_vec == [0x50, 0x4B, 0x03, 0x04] {
                    let index_as_usize: usize = match i.try_into() {
                        Ok(result) => result,
                        Err(_) => {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Could not convert",
                            ))
                        },
                    };
                    
                    local_file_headers_offsets.push((index_as_usize - 3) + counter)
                }
            }
    
            counter += buffer.len();
        }

        local_file_headers_offsets.push(counter);
    
        Ok(local_file_headers_offsets)
    }
}