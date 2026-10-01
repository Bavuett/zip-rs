pub mod error;
pub mod parser;
pub mod reader;
pub mod spec;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{BufReader, Read}};
    use super::*;

    // Legge un file dal disco e restituisce i suoi byte.
    // Usa CARGO_MANIFEST_DIR (la cartella radice del progetto) per trovare i file di test.
    fn read_file(path: &str) -> Vec<u8> {
        let file: File = match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("Impossibile aprire il file '{}': {}", path, e),
        };
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).expect("Errore nella lettura del file");
        bytes
    }

    // --- TESTS: is_valid_zip ---

    #[test]
    fn valid_zip_is_recognized() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sample.zip");
        let bytes = read_file(path);
        let mut parser = parser::Parser::new(&bytes);
        assert_eq!(parser.is_valid_zip().unwrap(), true);
    }

    #[test]
    fn empty_bytes_are_not_valid_zip() {
        let bytes: Vec<u8> = vec![];
        let mut parser = parser::Parser::new(&bytes);
        assert_eq!(parser.is_valid_zip().unwrap(), false);
    }

    #[test]
    fn too_small_file_is_not_valid_zip() {
        // Un file di 21 byte non può contenere un EOCD valido (minimo 22 byte)
        let bytes: Vec<u8> = vec![0u8; 21];
        let mut parser = parser::Parser::new(&bytes);
        assert_eq!(parser.is_valid_zip().unwrap(), false);
    }

    #[test]
    fn random_bytes_are_not_valid_zip() {
        let bytes: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x11, 0x22, 0x33];
        let mut parser = parser::Parser::new(&bytes);
        assert_eq!(parser.is_valid_zip().unwrap(), false);
    }

    // --- TESTS: get_end_of_central_directory ---

    #[test]
    fn eocd_total_entries_is_correct() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sample.zip");
        let bytes = read_file(path);
        let mut parser = parser::Parser::new(&bytes);
        let eocd = parser.get_end_of_central_directory().unwrap();
        // sample.zip contiene 1 solo file (hello.txt)
        assert_eq!(eocd.total_entries_in_central_directory, 1);
    }

    #[test]
    fn eocd_total_entries_multi_is_correct() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/multi.zip");
        let bytes = read_file(path);
        let mut parser = parser::Parser::new(&bytes);
        let eocd = parser.get_end_of_central_directory().unwrap();
        // multi.zip contiene 2 file (file_one.txt e folder/file_two.txt)
        assert_eq!(eocd.total_entries_in_central_directory, 2);
    }

    #[test]
    fn eocd_cd_offset_is_nonzero() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sample.zip");
        let bytes = read_file(path);
        let mut parser = parser::Parser::new(&bytes);
        let eocd = parser.get_end_of_central_directory().unwrap();
        // L'offset della Central Directory non può essere zero in un file non vuoto
        assert!(eocd.start_of_central_directory_offset > 0);
    }

    // --- TESTS: get_local_file_header ---

    #[test]
    fn local_file_header_file_name_is_correct() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sample.zip");
        let bytes = read_file(path);
        let mut parser = parser::Parser::new(&bytes);
        let header = parser.get_local_file_header().unwrap();
        // Il primo (e unico) file in sample.zip si chiama "hello.txt"
        assert_eq!(header.file_name, "hello.txt");
    }

    #[test]
    fn local_file_header_file_name_length_matches_name() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/sample.zip");
        let bytes = read_file(path);
        let mut parser = parser::Parser::new(&bytes);
        let header = parser.get_local_file_header().unwrap();
        // file_name_length deve corrispondere alla lunghezza effettiva del nome
        assert_eq!(header.file_name_length as usize, header.file_name.len());
    }
}
