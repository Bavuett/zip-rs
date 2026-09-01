use crate::archive::timestamps::last_modified_file_time::LastModifiedFileTime;

impl TryFrom<Vec<u8>> for LastModifiedFileTime {
    type Error = std::io::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let last_modified_file_time_bytes: u16 = u16::from_le_bytes([value[10], value[11]]);

        let hours = (last_modified_file_time_bytes >> 11) as u8;
        let minutes = ((last_modified_file_time_bytes >> 5) & 0x3F) as u8;
        let seconds = ((last_modified_file_time_bytes & 0x1F) * 2) as u8;

        Ok(LastModifiedFileTime {
            hours,
            minutes,
            seconds,
        })
    }
}
