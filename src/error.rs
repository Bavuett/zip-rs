pub enum ZipError {
    UnexpectedEof {
        offset: usize,
        needed: usize,
    },
    BadSignature {
        offset: usize,
        expected: u32,
        got: u32,
    },
    InvalidUtf8,
}
