pub mod utils;
pub mod archive;

#[cfg(test)]
mod tests {
    use super::utils::is_zip_file;
    use super::archive::Archive;

    #[test]
    fn it_works() {
        let file: std::fs::File = match std::fs::File::open("/home/bavuett/Zip Files/archive.zip") {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error),
        }; 
        
        let archive: Archive = Archive::open("/home/bavuett/Zip Files/archive.zip").expect("Error!");

        let is_zip = match is_zip_file(&file) {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error),
        };

        println!("Name: {} - Is ZIP: {}", archive.name, is_zip);
    }
}
