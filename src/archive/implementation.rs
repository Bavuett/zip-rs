use crate::{
    archive::{entry::Entry, Archive},
    factories::{
        entries::EntriesFactory,
        local_file_headers_offsets::LocalFileHeadersOffsetsFactory,
    },
    utils::validation::ValidationUtilities,
};

use std::io::{BufReader, Read, Seek, SeekFrom};

impl Archive {
    pub fn open(archive_path: &str) -> Result<Self, std::io::Error> {
        let mut buffer: [u8; 256] = [0; 256];

        let file_ref: std::fs::File = match std::fs::File::open(archive_path) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        let mut file: BufReader<std::fs::File> = BufReader::new(file_ref);

        let mut entries: Vec<Entry> = Vec::new();

        let is_zip: bool = match ValidationUtilities::is_zip_file(&mut file) {
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

        let local_file_headers_offsets: Vec<usize> = LocalFileHeadersOffsetsFactory::from(
            &mut file, 
            size
        ).expect("Could not get Headers!");

        println!("Local File Headers: {:?}", local_file_headers_offsets);

        for &local_file_header_offset in &local_file_headers_offsets {
            let offsets_as_iter_ref: &mut std::slice::Iter<'_, usize> = &mut local_file_headers_offsets.iter();
            let index: usize = offsets_as_iter_ref.position(|&current_offset| current_offset == local_file_header_offset).unwrap();
            let local_file_headers_offsets_length: usize = local_file_headers_offsets.len();

            if index < local_file_headers_offsets_length - 1 {
                println!(
                    "Length: {}, Index: {}",
                    local_file_headers_offsets.len(),
                    index
                );

                let next_index: usize = if index < local_file_headers_offsets.len() - 1 {
                    index + 1
                } else {
                    0
                };

                let entry: Entry = match EntriesFactory::from(
                    &mut file,
                    local_file_header_offset,
                    local_file_headers_offsets[next_index],
                ) {
                    Ok(result) => result,
                    Err(error) => return Err(error),
                };

                entries.push(entry);
            }
        }

        Ok(Archive { file, entries })
    }
}
