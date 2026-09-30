use crate::error::ZipError;

pub struct Lexer<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn seek(&mut self, pos: usize) -> Result<(), ZipError> {
        if pos > self.data.len() {
            return Err(ZipError::UnexpectedEof {
                offset: pos,
                needed: 0,
            });
        }

        self.pos = pos;
        Ok(())
    }

    pub fn read_bytes(&mut self, number: usize) -> Result<&'a [u8], ZipError> {
        let end: usize = {
            if self.pos + number > self.data.len() {
                return Err(ZipError::UnexpectedEof {
                    offset: self.pos,
                    needed: number,
                });
            }

            self.pos + number
        };

        let bytes: &[u8] = &self.data[self.pos..end];
        self.pos = end;
        Ok(bytes)
    }

    pub fn skip(&mut self, number: usize) -> Result<(), ZipError> {
        self.read_bytes(number)?;

        Ok(())
    }
}
