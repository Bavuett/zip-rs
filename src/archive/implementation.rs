use crate::{archive::Archive, utils::is_zip_file};

impl Archive {
    pub fn open(archive_path: &str) -> Result<Self, std::io::Error> {
        let file: std::fs::File = match std::fs::File::open(archive_path) {
            Ok(result) => result,
            Err(error) => return Err(error),
        };

        let is_zip: bool = match is_zip_file(&file) {
            Ok(result) => result,
            Err(error) => panic!("{}", error),
        };

        if !(is_zip) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Not a valid ZIP",
            ));
        }

        Ok(Archive {
            name: String::from("Hello, World!"),
        })
    }
}
