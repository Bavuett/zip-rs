pub mod utils;
pub mod archive;

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use super::utils::is_zip_file;
    use super::archive::Archive;

    #[test]
    fn it_works() {
        let file_ref: std::fs::File = match std::fs::File::open("/home/bavuett/Zip Files/archive2.zip") {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error),
        }; 

        let mut file: BufReader<std::fs::File> = BufReader::new(file_ref);
        
        let archive: Archive = Archive::open("/home/bavuett/Zip Files/archive2.zip").expect("Error!");

        let is_zip = match is_zip_file(&mut file) {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error),
        };

        println!("\nName: {} - Is ZIP: {}", archive.name, is_zip);

        
    }
}
