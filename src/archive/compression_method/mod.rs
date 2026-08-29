mod implementation;

#[derive(Debug)]
pub enum CompressionMethod {
    Store,
    Deflate,
    BZip2,
    LZMA,
    Unsupported(u16),
}
