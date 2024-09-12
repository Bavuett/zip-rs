use std::fs::{read, File};
use std::io::{BufReader, Bytes, Read, Seek, SeekFrom};
use std::path::Path;

pub fn get_file(path_as_str: &str) {
    println!("GET {}", path_as_str);

    let path: &Path = Path::new(path_as_str);

    if path.exists() {
        println!("File exists");
    } else {
        println!("File does not exist");
    }

    if path.is_file() {
        println!("Path is a file");

        // Print content of file as bytes shown in integer.
        let content: Vec<u8> = std::fs::read(path).unwrap();
        // let content_as_hex: Vec<String> = content.iter().map(|byte| format!("{:02X}", byte)).collect::<Vec<String>>();

        println!("{:?}", content);
    } else {
        println!("Path is not a file");
    }
}

pub fn is_zip_file(file: &mut BufReader<File>) -> std::io::Result<bool> {
    let zip_signature: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];

    let mut buffer: [u8; 4] = [0; 4];
    let bytes_read: usize = match file.read(&mut buffer) {
        Ok(result) => result,
        Err(error) => return Err(error),
    };

    println!("Bytes read: {}.\nBuffer: {:?}", bytes_read, buffer);

    if zip_signature == buffer {
        return Ok(true);
    }

    Ok(false)
}

pub fn get_local_file_headers_offsets(
    file: &mut BufReader<File>,
    size: u64,
) -> Result<Vec<u64>, std::io::Error> {
    let mut local_file_headers_offsets: Vec<u64> = Vec::new();
    let mut buffer: [u8; 256] = [0; 256];
    let mut counter: u64 = 0;

    _ = match file.seek(SeekFrom::Start(0)) {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    while counter < size {
        let mut check_vec: Vec<u8> = Vec::new();
        _ = match file.read(&mut buffer) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        println!("Current buffer: {:?}", buffer);

        for i in 0..buffer.len() {
            match buffer[i] {
                0x50 => {
                    check_vec.push(buffer[i]);
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
                let index_as_u64: u64 = match i.try_into() {
                    Ok(result) => result,
                    Err(_) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Could not convert",
                        ))
                    },
                };
                
                local_file_headers_offsets.push(index_as_u64 - 3)
            }
        }

        counter += 256;
    }

    Ok(local_file_headers_offsets)
}
