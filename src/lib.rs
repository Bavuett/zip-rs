pub mod utils;
pub mod archive;
pub mod factories;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::archive::Archive;

    #[test]
    fn it_works() {
        let archive: Archive = Archive::open("C:\\Users\\loren\\Documents\\text.zip").expect("Error!");
    }
}
