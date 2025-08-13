pub mod utils;
pub mod archive;
pub mod factories;
pub mod traits;

#[cfg(test)]
mod tests {
    use crate::{archive::flags::Flags, factories::flags::FlagsFactory};

    const BUFFER_AS_ARR: [u8; 256] = 
        [
            80, 75, 3, 4,   20, 0, 8, 0, 8, 0, 238, 105, 6, 91, 0, 0,
            0,  0,  8, 0,  8, 0, 238, 105, 6, 91, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 8, 0, 32, 0, 116, 101, 120, 116, 46, 116, 120, 116,
            117, 120, 11, 0, 1, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 85,
            84, 13, 0, 7, 81, 57, 147, 104, 81, 57, 147, 104, 81, 57, 147, 104,
            3, 0, 80, 75, 7, 8, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
            0, 0, 80, 75, 1, 2, 20, 3, 20, 0, 8, 0, 8, 0, 238, 105,
            6, 91, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 8, 0,
            24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 182, 129, 0, 0, 0, 0,
            116, 101, 120, 116, 46, 116, 120, 116, 117, 120, 11, 0, 1, 4, 0, 0,
            0, 0, 4, 0, 0, 0, 0, 85, 84, 5, 0, 1, 81, 57, 147, 104,
            80, 75, 5, 6, 0, 0, 0, 0, 1, 0, 1, 0, 78, 0, 0, 0,
            88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

    #[test]
    fn vec_is_zip() {
        use crate::traits::validatable::Validatable;

        let buffer: Vec<u8> = BUFFER_AS_ARR.to_vec();

        assert_eq!(buffer.is_zip(), true);
    }

    #[test]
    fn vec_is_not_zip() {
        use crate::traits::validatable::Validatable;

        let mut buffer: Vec<u8> = BUFFER_AS_ARR.to_vec();
        buffer[0] = 1;

        assert_ne!(buffer.is_zip(), true);
    }

    #[test]
    /* 
     * Using this buffer, only bit 3 should be toggled to 1. When expressed in u16, this should equal to 8 (2^3 = 8).
     * For context, about the ZIP General Purpose Bit Flags, see the PK ZIP Specification (APPNOTE.txt), which can be found at
     * https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT
     */
    fn flags_factory() {
        let buffer: Vec<u8> = BUFFER_AS_ARR.to_vec();
        let flags: Flags = FlagsFactory::from(&buffer).expect("Could not generate flags");

        assert_eq!(flags.as_u16_le(), 8);
    }
}
