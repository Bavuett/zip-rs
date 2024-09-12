use crate::{archive::Archive, utils::{get_local_file_headers_offsets, is_zip_file}};

use std::io::{BufReader, Read, Seek, SeekFrom};

impl Archive {
    pub fn open(archive_path: &str) -> Result<Self, std::io::Error> {
        let mut buffer: [u8; 256] = [0; 256];

        let file_ref: std::fs::File = match std::fs::File::open(archive_path) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        let mut file: BufReader<std::fs::File> = BufReader::new(file_ref);

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

        let size: u64 = match file.seek(SeekFrom::End(0)) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        file.read(&mut buffer).expect("Error!");

        let local_file_headers_offsets = get_local_file_headers_offsets(&mut file, size).expect("Could not get Headers!");

        println!("Local File Headers: {:?}", local_file_headers_offsets);

        Ok(Archive {
            file,
            name: String::from("Hello, World!"),
        })
    }
}
