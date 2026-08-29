use crate::archive::compression_method::CompressionMethod;

impl TryFrom<u16> for CompressionMethod {
    type Error = std::io::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => CompressionMethod::Store,
            8 => CompressionMethod::Deflate,
            12 => CompressionMethod::BZip2,
            14 => CompressionMethod::LZMA,
            other => CompressionMethod::Unsupported(other),
        })
    }
}

impl TryFrom<&Vec<u8>> for CompressionMethod {
    type Error = std::io::Error;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        // TODO
        Ok(CompressionMethod::Store)
    }
}
