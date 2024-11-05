use std::fs::File;
use std::io::{BufReader, Read};

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
