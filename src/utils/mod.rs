// Utilities module inside folder src/utils
use std::io::Read;
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

pub fn is_zip_file(mut file: &std::fs::File) -> std::io::Result<bool> {
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
