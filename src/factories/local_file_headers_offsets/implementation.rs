use crate::factories::local_file_headers_offsets::LocalFileHeadersOffsetsFactory;

use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

impl LocalFileHeadersOffsetsFactory {
    pub fn from(file: &mut BufReader<File>, size: u64) -> Result<Vec<usize>, std::io::Error> {
        let mut local_file_headers_offsets: Vec<usize> = Vec::new();
        let mut check_vec: Vec<u8> = Vec::new();

        let mut buffer: [u8; 256] = [0; 256];
        let mut counter: usize = 0;

        _ = match file.seek(SeekFrom::Start(0)) {
            Ok(_) => (),
            Err(error) => return Err(error),
        };

        while counter < size as usize {
            let bytes_read: usize = match file.read(&mut buffer) {
                Ok(result) => result,
                Err(error) => return Err(error),
            };

            // // EOF reached before "size" bytes were read (shouldn't normally happen if
            // "size" was computed correctly, but guards against an infinite loop instead
            // of scanning stale buffer contents.
            if bytes_read == 0 {
                break;
            }

            println!("Current buffer: {:?}", buffer);

            for i in 0..buffer.len() {
                match buffer[i] {
                    0x50 => {
                        check_vec.clear();
                        check_vec.push(0x50);
                    }
                    0x4B => {
                        if check_vec == [0x50] {
                            check_vec.push(0x4B);
                        } else {
                            check_vec.clear();
                        }
                    }
                    0x03 => {
                        if check_vec == [0x50, 0x4B] {
                            check_vec.push(0x03);
                        } else {
                            check_vec.clear();
                        }
                    }
                    0x04 => {
                        if check_vec == [0x50, 0x4B, 0x03] {
                            check_vec.push(0x04);
                        } else {
                            check_vec.clear();
                        }
                    }
                    _ => check_vec.clear(),
                }

                if check_vec == [0x50, 0x4B, 0x03, 0x04] {
                    // Use the absolute position in the file (`counter + i`)
                    // rather than the in-buffer index `i`. A signature can
                    // straddle two reads (e.g. "PK" at the end of one 256-byte
                    // chunk and "\x03\x04" at the start of the next), in which
                    // case `i` alone can be 0, 1 or 2 here and `i - 3` would
                    // underflow. The absolute position is always >= 3 once a
                    // full 4-byte signature has been matched.
                    let absolute_position: usize = counter + i;

                    local_file_headers_offsets.push(absolute_position - 3);

                    // A signature can't overlap itself, so there's no need to keep
                    // matching against its own tail bytes.
                    check_vec.clear();
                }
            }

            counter += buffer.len();
        }

        local_file_headers_offsets.push(counter);

        Ok(local_file_headers_offsets)
    }
}
