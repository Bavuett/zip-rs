pub mod utils;
pub mod archive;
pub mod factories;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::archive::Archive;

    #[test]
    fn it_works() {
        let archive: Archive = Archive::open("/home/bavuett/Zip Files/archive3.zip").expect("Error!");
    }
}
