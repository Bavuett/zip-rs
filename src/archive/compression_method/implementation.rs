use crate::{archive::compression_method::CompressionMethod, traits::validatable::Validatable};

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
        _ = match value.is_zip() {
            true => (),
            false => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Not a zip file",
                ))
            }
        };

        let compression_method_byte: u16 = u16::from_le_bytes([value[8], value[9]]);

        let compression_method: CompressionMethod =
            match self::CompressionMethod::try_from(compression_method_byte) {
                Ok(result) => result,
                Err(error) => return Err(error),
            };

        Ok(compression_method)
    }
}
