pub mod utils;

#[cfg(test)]
mod tests {
    use super::utils::is_zip_file;

    #[test]
    fn it_works() {
        let file: std::fs::File = match std::fs::File::open("/home/bavuett/Zip Files/archive.zip") {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error),
        }; 

        let is_zip: bool = match is_zip_file(file) {
            Ok(result) => result,
            Err(error) => panic!("{:?}", error),
        };

        println!("{}", is_zip);
    }
}
