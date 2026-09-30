pub enum ZipError {
    UnexpectedEof { offset: usize, needed: usize },
}
