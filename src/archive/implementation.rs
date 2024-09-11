use crate::{archive::Archive, utils::is_zip_file};

use std::io::{BufReader, Read, Seek, SeekFrom};

impl Archive {
    pub fn open(archive_path: &str) -> Result<Self, std::io::Error> {
        let mut buffer: [u8; 8] = [0; 8];

        let local_file_headers_offsets: Vec<u8> = Vec::from([0]);

        let file_ref: std::fs::File = match std::fs::File::open(archive_path) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        let mut file = BufReader::new(file_ref);

        let is_zip: bool = match is_zip_file(&mut file) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        if !(is_zip) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Not a valid ZIP",
            ));
        }

        let position: u64 = match file.seek(SeekFrom::Start(6)) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        file.read(&mut buffer).expect("Error!");

        println!("Reading from position {}: {:?}", position, buffer);

        Ok(Archive {
            file,
            name: String::from("Hello, World!"),
        })
    }
}
